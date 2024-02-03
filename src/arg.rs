use std::ops::{Range, RangeInclusive};

impl crate::BigRocket {
	/// 颜色范围: RGB
	pub(crate) const COLOR_RANGE: (RangeInclusive<u8>, RangeInclusive<u8>, RangeInclusive<u8>) =
		(0..=255, 0..=255, 0..=255);
	/// 质量
	pub(crate) const MASS: f64 = 50.;
	/// 水平初速度的范围
	pub(crate) const SPEED_RANGE_X: Range<f64> = -5.0..5.0;
	/// 竖直初速度的范围
	pub(crate) const SPEED_RANGE_Y: Range<f64> = 20.0..55.0;
	/// 轨迹扩散的范围
	pub(crate) const TRAIL_SPREAD: f64 = 0.5;
}

impl crate::SmallRocket {
	/// 寿命
	pub(crate) const AGE: f64 = 3.;
	/// 质量
	pub(crate) const MASS: f64 = 1.;
	/// 初速度的范围
	pub(crate) const SPEED_RANGE: Range<f64> = 5.0..150.0;
	/// 轨迹扩散的范围
	pub(crate) const TRAIL_SPREAD: f64 = 1.;
}

pub(crate) trait CanvasSize {
	fn width(&self) -> u32;
	fn height(&self) -> u32;
}
