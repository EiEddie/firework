use std::ops::{Range, RangeInclusive};

use crate::{Vec2, Vec2f, Vec2u};

/// 重力加速度
pub(crate) const G_PHY: f64 = 9.8;

/// 阻力常数
pub(crate) const DRAG_PHY: f64 = 0.1;

impl crate::rocket::BigRocket {
	/// 颜色范围: RGB
	pub(crate) const COLOR_RANGE: (RangeInclusive<u8>, RangeInclusive<u8>, RangeInclusive<u8>) =
		(0..=255, 0..=255, 0..=255);
	/// 质量
	pub(crate) const MASS: f64 = 50.;
	/// 水平初速度的范围
	pub(crate) const SPEED_RANGE_X: Range<f64> = -2.0..2.0;
	/// 竖直初速度的范围
	pub(crate) const SPEED_RANGE_Y: Range<f64> = 20.0..25.0;
	/// 轨迹扩散的范围
	pub(crate) const TRAIL_SPREAD: f64 = 0.5;
}

impl crate::rocket::SmallRocket {
	/// 寿命
	pub(crate) const AGE: f64 = 1.5;
	/// 质量
	pub(crate) const MASS: f64 = 1.;
	/// 初速度的范围
	pub(crate) const SPEED_RANGE: Range<f64> = 5.0..10.0;
	/// 轨迹扩散的范围
	pub(crate) const TRAIL_SPREAD: f64 = 1.;
}

impl crate::rocket::Particle {
	/// 寿命
	pub(crate) const AGE: f64 = 1.;
}

pub trait CanvasSize {
	fn width(&self) -> u32;
	fn height(&self) -> u32;

	fn phy_width(&self) -> f64 {
		self.width() as f64 / 2.
	}

	fn phy_height(&self) -> f64 {
		self.height() as f64
	}

	/// 从物理模拟的坐标位置转换为在终端上显示的列和行
	///
	/// - 将 `y` 轴倒置:
	///   因为物理坐标系原点在左下角, 而终端的原点在左上角
	/// - `x` 轴拉伸一倍:
	///   因为终端中一个字符的宽为高的 1/2
	fn to_col_and_row(&self, pos: &Vec2f) -> Option<Vec2u> {
		let p_x = (pos.0 * 2.).round();
		if 0. > p_x || p_x > self.width() as f64 - 1. {
			return None;
		}

		let p_y = self.height() as f64 - pos.1.round();
		if 0. > p_y || p_y > self.height() as f64 - 1. {
			return None;
		}

		Some(Vec2(p_x as u32, p_y as u32))
	}
}
