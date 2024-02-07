pub mod arg;
pub mod rocket;
pub mod time;

use std::cell::{RefCell, RefMut};

use rand::rngs::ThreadRng;

thread_local! {
	/// 单线程全局随机数发生器
	static RNG: RefCell<ThreadRng> = RefCell::new(rand::thread_rng());
}

/// 调用一个全局随机数生成器
///
/// # Example
///
/// ```
/// let r: f64 = rng_do(|rng| rng.gen());
/// ```
fn rng_do<F, R>(f: F) -> R
	where F: FnOnce(&mut RefMut<'_, ThreadRng>) -> R
{
	RNG.with(|rng| f(&mut rng.borrow_mut()))
}

use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T: Copy>(pub T, pub T);

pub type Vec2f = Vec2<f64>;
pub type Vec2u = Vec2<u32>;

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

impl std::fmt::Display for Vec2<f64> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({:.3}, {:.3})", self.0, self.1)?;
		Ok(())
	}
}

impl std::fmt::Display for Vec2<i32> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({:>3}, {:>3})", self.0, self.1)?;
		Ok(())
	}
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
