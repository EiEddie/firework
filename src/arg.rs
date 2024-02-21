/// 在程序运行时所需的常量
///
/// 物理常量, 模拟参数与程序控制相关的参数
use std::f64::consts::PI;
use std::ops::{Range, RangeInclusive};

/// 大火箭升空的频率 `[Hz]`
pub const BIG_ROCKET_LAUNCH_FREQ: f64 = 1.;

/// 程序运行的预期帧率
pub const FPS: f64 = 60.;

/// 粒子生成的频率 `[Hz]`
///
/// 每个烟花 (无论大小) 每秒生成的发光粒子数
pub const GLITTER_GENERATE_FREQ: f64 = 50.;

/// 呈现使用的字符
pub const DISPLAY_CHAR: char = '█';

/// 背景颜色: RGB
/// NOTE: 目前颜色只能为黑色
pub const BACKGROUND_COLOR: (u8, u8, u8) = (0, 0, 0);

/// 重力加速度 `[m/s^2]`
pub(crate) const G_PHY: f64 = 9.8;

/// 阻力常数 `[kg/s]`
pub(crate) const DRAG_PHY: f64 = 0.1;

impl crate::rocket::BigRocket {
	/// 颜色范围: RGB
	pub(crate) const COLOR_RANGE: (RangeInclusive<u8>, RangeInclusive<u8>, RangeInclusive<u8>) =
		(0..=255, 0..=255, 0..=255);
	/// 质量 `[kg]`
	pub(crate) const MASS: f64 = 50.;
	/// 水平初速度的范围 `[m/s]`
	pub(crate) const SPEED_RANGE_X: Range<f64> = -2.0..2.0;
	/// 竖直初速度的范围 `[m/s]`
	pub(crate) const SPEED_RANGE_Y: Range<f64> = 20.0..25.0;
	/// 轨迹扩散的范围 `[m]`
	pub(crate) const TRAIL_SPREAD: f64 = 0.5;
}

impl crate::rocket::SmallRocket {
	/// 寿命 `[s]`
	pub(crate) const AGE: f64 = 1.5;
	/// 爆炸时不会出现的范围 `[rad]`
	///
	/// 避免烟花向底部炸开, 圆形底部留出一个扇形空间
	/// 扇形里面将不会出现烟花
	/// 扇形的弧度
	pub(crate) const EXPLODE_EXCLUDE_ANGLE: f64 = PI / 8.;
	/// 每次爆炸时生成的小火箭的数量
	pub(crate) const EXPLODE_ROCKET_CNT: u32 = 25;
	/// 质量 `[kg]`
	pub(crate) const MASS: f64 = 1.;
	/// 初速度的范围 `[m/s]`
	pub(crate) const SPEED_RANGE: Range<f64> = 5.0..10.0;
	/// 轨迹扩散的范围 `[m]`
	pub(crate) const TRAIL_SPREAD: f64 = 1.;
}

impl crate::rocket::Particle {
	/// 寿命 `[s]`
	pub(crate) const AGE: f64 = 1.;
}
