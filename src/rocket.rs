use std::f64::consts::TAU;

use colorsys::Rgb;
use rand::Rng;

use crate::arg::{self, *};
use crate::{rng_do, Vec2, Vec2f, Vec2i};

#[derive(Debug)]
pub(crate) struct BigRocket {
	/// 位置
	pos: Vec2f,
	/// 质量
	mas: f64,
	/// 速度
	vel: Vec2f,

	/// 颜色
	color: Rgb,

	/// 扩散范围
	/// 可以制作模糊的轨迹
	spread: f64,
}

#[derive(Debug)]
pub(crate) struct SmallRocket {
	/// 位置
	pos: Vec2f,
	/// 质量
	mas: f64,
	/// 速度
	vel: Vec2f,

	/// 颜色
	color: Rgb,

	/// 扩散范围
	/// 可以制作模糊的轨迹
	spread: f64,

	/// 寿命
	age: f64,
}

impl BigRocket {
	fn launch(size: &impl arg::CanvasSize) -> Self {
		let p_x = rng_do(|rng| rng.gen_range(0.0..size.width() as f64));

		let v_x = rng_do(|rng| rng.gen_range(Self::SPEED_RANGE_X));
		let v_y = rng_do(|rng| rng.gen_range(Self::SPEED_RANGE_Y));

		let c_r = rng_do(|rng| rng.gen_range(Self::COLOR_RANGE.0));
		let c_g = rng_do(|rng| rng.gen_range(Self::COLOR_RANGE.1));
		let c_b = rng_do(|rng| rng.gen_range(Self::COLOR_RANGE.2));

		Self { pos:    Vec2(p_x, 0.),
		       mas:    Self::MASS,
		       vel:    Vec2(v_x, v_y),
		       color:  Rgb::from((c_r, c_g, c_b)),
		       spread: Self::TRAIL_SPREAD, }
	}

	/// 大烟花火箭爆炸
	///
	/// 一个大的火箭爆炸时会生成 `cnt` 个小火箭, 储存在 `dst` 内
	///
	/// # Returns
	///
	/// 返回火箭是否爆炸
	fn explode(&self, cnt: u32, dst: &mut Vec<SmallRocket>) -> bool {
		// 火箭还在上升
		// 升到最高点爆炸
		if self.vel.1 > 0. {
			return false;
		}

		for _ in 0..cnt {
			dst.push(SmallRocket::from_big_rocket(self));
		}
		return true;
	}
}

impl SmallRocket {
	fn from_big_rocket(big_rocket: &BigRocket) -> Self {
		let v_norm = rng_do(|rng| rng.gen_range(Self::SPEED_RANGE));
		let v_deg = rng_do(|rng| rng.gen_range(0.0..TAU));

		Self { pos:    big_rocket.pos,
		       mas:    Self::MASS,
		       vel:    Vec2(v_deg.cos(), v_deg.sin()) * v_norm,
		       color:  big_rocket.color.clone(),
		       spread: Self::TRAIL_SPREAD,
		       age:    Self::AGE, }
	}
}

/// 烟花释放的发光粒子
///
/// 可显示在屏幕上
#[derive(Debug)]
pub struct Particle {
	/// 位置
	pos: Vec2i,

	/// 颜色
	color: Rgb,

	/// 持续时间
	age: f64,
}

impl Particle {
	pub fn pos(&self) -> Vec2i {
		self.pos
	}

	pub fn color(&self) -> Rgb {
		self.color.clone()
	}

	// FIXME: 物理坐标系和显示坐标系不一致, 物理坐标系原点在左下角, 而显示坐标系在左上角

	fn try_from_big_rocket(big_rocket: &BigRocket, size: &impl CanvasSize) -> Option<Self> {
		// 逆时针旋转 90 deg 的速度
		let v_ver = Vec2(big_rocket.vel.1, -big_rocket.vel.0);

		// 归一化后乘 轨迹扩散的范围 得到坐标的偏移量
		let p_delta = v_ver
		              * (1. / (v_ver.0.powi(2) + v_ver.1.powi(2)).sqrt()
		                 * rng_do(|rng| rng.gen_range(-big_rocket.spread..big_rocket.spread)));

		let p = p_delta + big_rocket.pos;

		let p_x = p.0 as i32;
		if 0 > p_x || p_x > size.width() as i32 {
			return None;
		}

		let p_y = p.1 as i32;
		if 0 > p_y || p_y > size.height() as i32 {
			return None;
		}

		Some(Self { pos:   Vec2(p_x, p_y),
		       color: big_rocket.color.clone(),
		       age:   Self::AGE, })
	}

	fn try_from_small_rocket(small_rocket: &SmallRocket, size: &impl CanvasSize) -> Option<Self> {
		// 逆时针旋转 90 deg 的速度
		let v_ver = Vec2(small_rocket.vel.1, -small_rocket.vel.0);

		// 归一化后乘 轨迹扩散的范围 得到坐标的偏移量
		let p_delta = v_ver
		              * (1. / (v_ver.0.powi(2) + v_ver.1.powi(2)).sqrt()
		                 * rng_do(|rng| rng.gen_range(-small_rocket.spread..small_rocket.spread)));

		let p = p_delta + small_rocket.pos;

		let p_x = p.0 as i32;
		if 0 > p_x || p_x > size.width() as i32 {
			return None;
		}

		let p_y = p.1 as i32;
		if 0 > p_y || p_y > size.height() as i32 {
			return None;
		}

		// TODO: 随寿命降低, 发出的光逐渐变暗

		Some(Self { pos:   Vec2(p_x, p_y),
		       color: small_rocket.color.clone(),
		       age:   Self::AGE, })
	}
}

/// 管理和更新所有可见粒子与烟花
pub struct Glitters {
	big_rockets:   Vec<BigRocket>,
	small_rockets: Vec<SmallRocket>,
	particles:     Vec<Particle>,
}

impl Glitters {
	pub fn new() -> Self {
		Self { big_rockets:   Vec::new(),
		       small_rockets: Vec::new(),
		       particles:     Vec::new(), }
	}

	/// 发射一枚大火箭
	pub fn launch(&mut self, size: &impl arg::CanvasSize) {
		self.big_rockets.push(BigRocket::launch(size));
	}

	/// 烟花的更新
	///
	/// - 大小烟花寿命的减少
	/// - 大小烟花的运动
	/// - 大烟花的爆炸:
	///   爆炸后移除此烟花, 并生成若干个小烟花
	/// - 删除超出寿命的小烟花
	pub fn update_rockets(&mut self, dt: f64) {
		// 删除气数已尽的小烟花
		self.small_rockets.retain(|x| x.age > 0.);

		// 移除符合条件的爆炸的大烟花
		self.big_rockets
		    .retain(|x| !x.explode(30, &mut self.small_rockets));

		// 大烟花的运动
		for brkt in &mut self.big_rockets {
			brkt.pos += brkt.vel * dt;
			brkt.vel += Vec2(0., -G_PHY * dt);
		}

		// 小烟花的运动
		for srkt in &mut self.small_rockets {
			srkt.pos += srkt.vel * dt;
			// 小烟花受阻力:
			// f = -kv => ma = -g - kv => v = v_0 + (-g/m - k/m*v)*dt
			let a_f = srkt.vel * (-DRAG_PHY / srkt.mas);
			srkt.vel += (Vec2(0., -G_PHY / srkt.mas) + a_f) * dt;

			srkt.age -= dt;
		}
	}

	/// 可见粒子寿命的更新
	///
	/// - 减少粒子的寿命
	/// - 尾迹的消失:
	///   删除超出寿命的粒子
	pub fn update_glitters(&mut self, dt: f64) {
		// 移除超出寿命的尾迹
		self.particles.retain(|x| x.age > 0.);

		// 减少粒子寿命
		for part in &mut self.particles {
			part.age -= dt;
		}
	}

	/// 生成可见粒子
	///
	/// - 生成大小火箭的尾迹
	/// - 小火箭尾迹的淡出:
	///   临近寿命的小火箭会更少地生成粒子
	pub fn gen_glitters(&mut self, size: &impl arg::CanvasSize) {
		// 为小火箭生成尾迹
		for srkt in &self.small_rockets {
			// 根据给定的函数对应的概率生成
			// 当火箭寿命将尽时, 将生成更少的尾迹
			// TODO: 先判断寿命是否大于 1 可以减少平方根的开销
			if rng_do(|rng| rng.gen::<f64>()) < srkt.age.sqrt().clamp(0., 1.) {
				if let Some(part) = Particle::try_from_small_rocket(srkt, size) {
					self.particles.push(part);
				}
			}
		}

		// 为大火箭生成尾迹
		for brkt in &self.big_rockets {
			if let Some(part) = Particle::try_from_big_rocket(brkt, size) {
				self.particles.push(part);
			}
		}
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Particle> {
		self.particles.iter()
	}
}
