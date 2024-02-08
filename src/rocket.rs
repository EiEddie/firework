use std::collections::BTreeMap;
use std::f64::consts::PI;

use colorsys::{Hsl, Rgb};
use rand::Rng;

use crate::arg::*;
use crate::{rng_do, CanvasSize, Vec2, Vec2f, Vec2u};

#[derive(Debug)]
pub(crate) struct BigRocket {
	/// 位置
	pos: Vec2f,
	/// 质量
	#[allow(dead_code)]
	mas: f64,
	/// 速度
	vel: Vec2f,

	/// 颜色
	color: Hsl,

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
	color: Hsl,

	/// 扩散范围
	/// 可以制作模糊的轨迹
	spread: f64,

	/// 寿命
	age: f64,
}

impl BigRocket {
	fn launch(size: &impl CanvasSize) -> Self {
		let p_x = rng_do(|rng| rng.gen_range(0.0..size.phy_width() as f64));

		let v_x = rng_do(|rng| rng.gen_range(Self::SPEED_RANGE_X));
		let v_y = rng_do(|rng| rng.gen_range(Self::SPEED_RANGE_Y));

		let c_r = rng_do(|rng| rng.gen_range(Self::COLOR_RANGE.0));
		let c_g = rng_do(|rng| rng.gen_range(Self::COLOR_RANGE.1));
		let c_b = rng_do(|rng| rng.gen_range(Self::COLOR_RANGE.2));

		Self { pos:    Vec2(p_x, 0.),
		       mas:    Self::MASS,
		       vel:    Vec2(v_x, v_y),
		       color:  Rgb::from((c_r, c_g, c_b)).into(),
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

		let v_deg = rng_do(|rng| {
			rng.gen_range(
			              (Self::EXPLODE_EXCLUDE_ANGLE - PI) / 2.0
			              ..(3. * PI - Self::EXPLODE_EXCLUDE_ANGLE) / 2.0,
			)
		});

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
	pos: Vec2u,

	/// 颜色
	color: Hsl,

	/// 持续时间
	age: f64,
}

impl Particle {
	pub fn pos(&self) -> Vec2u {
		self.pos
	}

	pub fn color(&self) -> Rgb {
		self.color.clone().into()
	}

	fn try_from_big_rocket(big_rocket: &BigRocket, size: &impl CanvasSize) -> Option<Self> {
		// 逆时针旋转 90 deg 的速度
		let v_ver = Vec2(big_rocket.vel.1, -big_rocket.vel.0);

		// 归一化后乘 轨迹扩散的范围 得到坐标的偏移量
		let p_delta = v_ver
		              * (1. / (v_ver.0.powi(2) + v_ver.1.powi(2)).sqrt()
		                 * rng_do(|rng| rng.gen_range(-big_rocket.spread..big_rocket.spread)));

		let p = p_delta + big_rocket.pos;

		let mut color = big_rocket.color.clone();
		color.set_lightness(90.);

		Some(Self { pos: size.to_col_and_row(&p)?,
		            color,
		            age: Self::AGE })
	}

	fn try_from_small_rocket(small_rocket: &SmallRocket, size: &impl CanvasSize) -> Option<Self> {
		// 逆时针旋转 90 deg 的速度
		let v_ver = Vec2(small_rocket.vel.1, -small_rocket.vel.0);

		// 归一化后乘 轨迹扩散的范围 得到坐标的偏移量
		let p_delta = v_ver
		              * (1. / (v_ver.0.powi(2) + v_ver.1.powi(2)).sqrt()
		                 * rng_do(|rng| rng.gen_range(-small_rocket.spread..small_rocket.spread)));

		let p = p_delta + small_rocket.pos;

		// l 范围: [60, 95]
		// 使用线性插值算法
		let mut color = small_rocket.color.clone();
		color.set_lightness(60. + (95. - 60.) * (small_rocket.age / SmallRocket::AGE));

		Some(Self { pos: size.to_col_and_row(&p)?,
		            color,
		            age: Self::AGE })
	}
}

/// 管理和更新所有可见粒子与烟花
pub struct Glitters {
	big_rockets:   Vec<BigRocket>,
	small_rockets: Vec<SmallRocket>,
	/// 一个有序 Map, 储存发光粒子的位置, 寿命与颜色
	///
	/// 0. `key`: 位置
	/// 1. `val`: 一个 pair, 分别是
	///     0. [`f64`] : 寿命
	///     1. [`Hsl`] : 颜色, 以 `HSL` 色彩空间存储
	particles:     BTreeMap<Vec2u, (f64, Hsl)>,
}

impl Glitters {
	pub fn new() -> Self {
		Self { big_rockets:   Vec::new(),
		       small_rockets: Vec::new(),
		       particles:     BTreeMap::new(), }
	}

	#[cfg(debug_assertions)]
	pub fn cnt_small_rocket(&self) -> u32 {
		self.small_rockets.len() as u32
	}

	#[cfg(debug_assertions)]
	pub fn cnt_big_rocket(&self) -> u32 {
		self.big_rockets.len() as u32
	}

	#[cfg(debug_assertions)]
	pub fn cnt_particle(&self) -> u32 {
		self.particles.len() as u32
	}

	/// 发射一枚大火箭
	pub fn launch(&mut self, size: &impl CanvasSize) {
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
		    .retain(|x| !x.explode(SmallRocket::EXPLODE_ROCKET_CNT, &mut self.small_rockets));

		// 大烟花的运动
		for brkt in &mut self.big_rockets {
			brkt.pos += brkt.vel * dt;
			brkt.vel += Vec2(0., -G_PHY * dt);
		}

		// 小烟花的运动
		for srkt in &mut self.small_rockets {
			srkt.pos += srkt.vel * dt;
			// 小烟花受阻力:
			// f = -kv => ma = -mg - kv => v = v_0 + (-g - k/m*v)*dt
			let a_f = srkt.vel * (-DRAG_PHY / srkt.mas);
			srkt.vel += (Vec2(0., -G_PHY) + a_f) * dt;

			// 烟花速度越快, 温度衰减越快, 寿命越低
			// 所以速度快的烟花的寿命会更快耗尽
			let k = 0.1;
			srkt.age -= k * (srkt.vel.0.powi(2) + srkt.vel.1.powi(2)).sqrt() * dt;
		}
	}

	/// 可见粒子寿命的更新
	///
	/// - 尾迹逐渐变暗
	/// - 减少粒子的寿命
	/// - 尾迹的消失:
	///   删除超出寿命的粒子
	pub fn update_glitters(&mut self, dt: f64) {
		// 移除超出寿命的尾迹
		self.particles.retain(|_, (age, _)| *age > 0.);

		// 减少粒子寿命
		for (_, (age, color)) in &mut self.particles {
			*age -= dt;

			let k = 150.;
			color.set_lightness(color.lightness() - k * (-(*age)).exp() * dt);
		}
	}

	/// 生成可见粒子
	///
	/// - 生成大小火箭的尾迹
	/// - 小火箭尾迹的淡出:
	///   临近寿命的小火箭会更少地生成粒子
	pub fn gen_glitters(&mut self, size: &impl CanvasSize) {
		// 为小火箭生成尾迹
		for srkt in &self.small_rockets {
			// 根据给定的函数对应的概率生成
			// 当火箭寿命将尽时, 将生成更少的尾迹
			// TODO: 先判断寿命是否大于 1 可以减少平方根的开销
			if rng_do(|rng| rng.gen::<f64>()) < srkt.age.sqrt().clamp(0., 1.) {
				if let Some(part) = Particle::try_from_small_rocket(srkt, size) {
					self.particles.insert(part.pos, (part.age, part.color));
				}
			}
		}

		// 为大火箭生成尾迹
		for brkt in &self.big_rockets {
			if let Some(part) = Particle::try_from_big_rocket(brkt, size) {
				self.particles.insert(part.pos, (part.age, part.color));
			}
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = (&Vec2u, Rgb)> {
		self.particles
		    .iter()
		    .map(|(pos, (_, color))| (pos, Rgb::from(color)))
	}
}
