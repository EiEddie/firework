use colorsys::{Hsl, Rgb};
use rand::{self, Rng};
mod arg;
use arg::*;

#[derive(Debug)]
struct Vec2<T>(T, T);

type Vec2f = Vec2<f64>;
type Vec2i = Vec2<i32>;

#[derive(Debug)]
struct Rocket {
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

	is_small: bool,
	/// 如果是火箭的小颗粒
	/// 则它存在寿命
	age:      f64,
}

impl Rocket {
	fn new_big(size: &impl arg::CanvasSize) -> Self {
		let mut rng = rand::thread_rng();

		let p_x = rng.gen_range(0.0..size.width() as f64);

		let v_x = rng.gen_range(-BIG_ROCKET_SPEED_X..BIG_ROCKET_SPEED_X);
		let v_y = -rng.gen_range(BIG_ROCKET_SPEED_Y[0]..BIG_ROCKET_SPEED_Y[1]);

		let c_r = rng.gen_range(BIG_ROCKET_COLOR_RANGE[0].0..BIG_ROCKET_COLOR_RANGE[1].0);
		let c_g = rng.gen_range(BIG_ROCKET_COLOR_RANGE[0].1..BIG_ROCKET_COLOR_RANGE[1].1);
		let c_b = rng.gen_range(BIG_ROCKET_COLOR_RANGE[0].2..BIG_ROCKET_COLOR_RANGE[1].2);

		Self { pos:      Vec2(p_x, 0.),
		       mas:      BIG_ROCKET_MASS,
		       vel:      Vec2(v_x, v_y),
		       color:    Rgb::from((c_r, c_g, c_b)).into(),
		       spread:   BIG_ROCKET_TRAIL_SPREAD,
		       is_small: false,
		       age:      f64::MAX, }
	}
}
