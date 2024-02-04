#![allow(dead_code)]
// TODO

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
		let v_y = -rng_do(|rng| rng.gen_range(Self::SPEED_RANGE_Y));

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
pub(crate) struct Particle {
	/// 位置
	pos: Vec2i,

	/// 颜色
	color: Rgb,

	/// 持续时间
	age: f64,
}

impl Particle {
	fn from_big_rocket(big_rocket: &BigRocket, size: &impl CanvasSize) -> Self {
		let p_x = (big_rocket.pos.0 as i32).clamp(0, size.width() as i32);
		let p_y = (big_rocket.pos.1 as i32).clamp(0, size.height() as i32);

		Self { pos:   Vec2(p_x, p_y),
		       color: big_rocket.color.clone(),
		       age:   Self::AGE, }
	}

	fn from_small_rocket(small_rocket: &SmallRocket, size: &impl CanvasSize) -> Self {
		let p_x = (small_rocket.pos.0 as i32).clamp(0, size.width() as i32);
		let p_y = (small_rocket.pos.1 as i32).clamp(0, size.height() as i32);

		// todo: 提高明度

		Self { pos:   Vec2(p_x, p_y),
		       color: small_rocket.color.clone(),
		       age:   Self::AGE, }
	}
}

/// 管理和更新所有可见粒子与烟花
struct Glitters {
	big_rockets:   Vec<BigRocket>,
	small_rockets: Vec<SmallRocket>,
	particles:     Vec<Particle>,
}

impl Glitters {
	fn new() -> Self {
		Self { big_rockets:   Vec::new(),
		       small_rockets: Vec::new(),
		       particles:     Vec::new(), }
	}

	/// 烟花的更新
	///
	/// - 大小烟花的运动
	/// - 大烟花的爆炸
	/// - 删除超出寿命的小烟花
	fn update_rockets(&mut self, dt: f64) {
		// 删除气数已尽的小烟花
		self.small_rockets.retain(|x| x.age > 0.);

		// 移除符合条件的爆炸的大烟花
		// FIXME: 注意测试
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

		// todo: 生成可见元素
	}
}
