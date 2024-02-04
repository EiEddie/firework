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
pub struct Vec2<T: Copy>(T, T);

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
