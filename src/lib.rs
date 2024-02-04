mod arg;
mod rocket;

use std::ops::{Add, AddAssign, Mul};

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
