/// 大火箭的质量
pub(crate) const BIG_ROCKET_MASS: f64 = 50.;

/// 小火箭的质量
pub(crate) const SMALL_ROCKET_MASS: f64 = 1.;

/// 大火箭初速度的范围
pub(crate) const BIG_ROCKET_SPEED_Y: [f64; 2] = [20., 55.];
pub(crate) const BIG_ROCKET_SPEED_X: f64 = 5.;

/// 小火箭初速度的范围
pub(crate) const SMALL_ROCKET_SPEED: [f64; 2] = [5., 150.];

/// 大火箭轨迹扩散的范围
pub(crate) const BIG_ROCKET_TRAIL_SPREAD: f64 = 0.5;

/// 小火箭轨迹扩散的范围
pub(crate) const SAMLL_ROCKET_TRAIL_SPREAD: f64 = 1.;

/// 大火箭颜色范围: RGB
pub(crate) const BIG_ROCKET_COLOR_RANGE: [(u8, u8, u8); 2] = [(50, 100, 70), (70, 100, 100)];

/// 小火箭寿命
pub(crate) const SMALL_ROCKET_AGE: f64 = 3.;

pub(crate) trait CanvasSize {
	fn width(&self) -> u32;
	fn height(&self) -> u32;
}
