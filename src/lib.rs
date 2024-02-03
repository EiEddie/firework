use std::f64::consts::TAU;
use std::ops::{Add, AddAssign, Mul};

use colorsys::Rgb;
use rand::{self, Rng};
mod arg;
use arg::*;

#[derive(Debug, Clone, Copy)]
struct Vec2<T>(T, T);

type Vec2f = Vec2<f64>;
type Vec2i = Vec2<i32>;

impl Mul<f64> for Vec2<f64> {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output {
		Vec2(self.0 * rhs, self.1 * rhs)
	}
}

impl Add for Vec2<f64> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Vec2(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl AddAssign for Vec2<f64> {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

#[derive(Debug)]
struct BigRocket {
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
struct SmallRocket {
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
		       color:  Rgb::from((c_r, c_g, c_b)),
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
	fn new(pos: Vec2f, color: Rgb) -> Self {
		let mut rng = rand::thread_rng();

		let v_norm = rng.gen_range(Self::SPEED_RANGE);
		let v_deg = rng.gen_range(0.0..TAU);

		Self { pos,
		       mas: Self::MASS,
		       vel: Vec2(v_deg.cos(), v_deg.sin()) * v_norm,
		       color,
		       spread: Self::TRAIL_SPREAD,
		       age: Self::AGE }
	}
}

/// 烟花释放的发光粒子
///
/// 可显示在屏幕上
#[derive(Debug)]
struct Particle {
	/// 位置
	pos: Vec2i,

	/// 颜色
	color: Rgb,

	/// 持续时间
	age: f64,
}

impl Particle {
	fn new(pos: Vec2f, color: Rgb, size: &impl CanvasSize) -> Self {
		let p_x = (pos.0 as i32).clamp(0, size.width() as i32);
		let p_y = (pos.1 as i32).clamp(0, size.height() as i32);

		// todo: 提高明度

		Self { pos: Vec2(p_x, p_y),
		       color,
		       age: Self::AGE }
	}
}

/// 管理和更新所有可见粒子与烟花
struct Glitters {
	big_rockets:   Vec<BigRocket>,
	small_rockets: Vec<SmallRocket>,
	particles:     Vec<Particle>,
}

impl Glitters {
	/// 燃放 `cnt` 数量的烟花, 开始模拟
	fn new(cnt: u32, size: &impl arg::CanvasSize) -> Self {
		let mut big_rockets: Vec<BigRocket> = Vec::new();
		for _ in 0..cnt {
			big_rockets.push(BigRocket::launch(size));
		}

		Self { big_rockets,
		       small_rockets: Vec::new(),
		       particles: Vec::new() }
	}

	fn update(&mut self, dt: f64) {
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
