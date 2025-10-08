use num_traits::Zero;
use std::{
	cmp::{PartialEq, PartialOrd},
	marker::Copy,
	ops::Add,
};

pub struct Triangle<T>(T, T, T);

impl<T: PartialOrd + PartialEq + Add<Output = T> + Zero + Copy> Triangle<T> {
	pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
		let [a, b, c] = sides;
		// All sides must be > 0
		if !(a > T::zero() && b > T::zero() && c > T::zero()) {
			return None;
		}

		if !(a + b >= c && b + c >= a && c + a >= b) {
			return None;
		}

		Some(Triangle(a, b, c))
	}

	pub fn is_equilateral(&self) -> bool {
		self.0 == self.1 && self.1 == self.2
	}

	pub fn is_scalene(&self) -> bool {
		self.0 != self.1 && self.0 != self.2 && self.1 != self.2
	}

	pub fn is_isosceles(&self) -> bool {
		self.0 == self.1 || self.1 == self.2 || self.2 == self.0
	}
}
