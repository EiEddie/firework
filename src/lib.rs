use std::f64::consts::TAU;

use colorsys::{Hsl, Rgb};
use rand::{self, Rng};
mod arg;

#[derive(Debug, Clone, Copy)]
struct Vec2<T>(T, T);

type Vec2f = Vec2<f64>;
type Vec2i = Vec2<i32>;

#[derive(Debug)]
struct BigRocket {
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
}

#[derive(Debug)]
struct SmallRocket {
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
	fn launch(size: &impl arg::CanvasSize) -> Self {
		let mut rng = rand::thread_rng();

		let p_x = rng.gen_range(0.0..size.width() as f64);

		let v_x = rng.gen_range(Self::SPEED_RANGE_X);
		let v_y = -rng.gen_range(Self::SPEED_RANGE_Y);

		let c_r = rng.gen_range(Self::COLOR_RANGE.0);
		let c_g = rng.gen_range(Self::COLOR_RANGE.1);
		let c_b = rng.gen_range(Self::COLOR_RANGE.2);

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
	/// Returns
	///
	/// 返回火箭是否爆炸
	fn explode(&self, cnt: u32, dst: &mut Vec<SmallRocket>) -> bool {
		// 火箭还在上升
		// 升到最高点爆炸
		if self.vel.1 > 0. {
			return false;
		}

		for _ in 0..cnt {
			dst.push(SmallRocket::new(self.pos, self.color.clone()));
		}
		return true;
	}
}

impl SmallRocket {
	fn new(pos: Vec2f, color: Hsl) -> Self {
		let mut rng = rand::thread_rng();

		let v_norm = rng.gen_range(Self::SPEED_RANGE);
		let v_deg = rng.gen_range(0.0..TAU);

		Self { pos,
		       mas: Self::MASS,
		       vel: Vec2(v_norm * v_deg.cos(), v_norm * v_deg.sin()),
		       color,
		       spread: Self::TRAIL_SPREAD,
		       age: Self::AGE }
	}
}
