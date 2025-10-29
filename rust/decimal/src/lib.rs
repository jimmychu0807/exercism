use std::cmp::PartialEq;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug)]
pub struct Decimal {
	positive: bool,
	digits: Vec<u8>,
	decimal: usize,
}

impl Decimal {
	pub fn try_from(input: &str) -> Option<Decimal> {
		let mut digits = Vec::new();
		let mut decimal: Option<usize> = None;
		let mut positive = true;

		for (idx, chr) in input.chars().enumerate() {
			if idx == 0 && chr == '-' {
				positive = false;
				continue;
			}

			match chr {
				'0'..='9' => {
					digits.push(chr as u8 - b'0');
				}
				'.' => {
					decimal = Some(idx);
				}
				_ => {
					return None;
				}
			}
		}

		Some(Decimal { positive, digits, decimal: decimal.unwrap_or(input.len() - 1) })
	}
}

impl PartialEq for Decimal {
	fn eq(&self, other: &Decimal) -> bool {
		// algorithm
		if self.positive != other.positive {
			return false;
		}

		// everything in front of the decimal must be the same to be Eq
		if self.decimal != other.decimal {
			return false;
		}

		// check the integer part
		let self_integer = &self.digits[0..self.decimal];
		let other_integer = &other.digits[0..other.decimal];
		if self_integer.iter().zip(other_integer.iter()).any(|(s_v, o_v)| s_v != o_v) {
			return false;
		}

		// check the fractional part
		let mut self_fractional: Vec<_> = if self.decimal < self.digits.len() - 1 {
			self.digits[self.decimal + 1..].to_vec()
		} else {
			Vec::new()
		};
		let mut other_fractional: Vec<_> = if other.decimal < other.digits.len() - 1 {
			other.digits[other.decimal + 1..].to_vec()
		} else {
			Vec::new()
		};
		// expand the short one by padding zero to the end
		let (short_vec, long_vec) = if self_fractional.len() < other_fractional.len() {
			(&mut self_fractional, &mut other_fractional)
		} else {
			(&mut other_fractional, &mut self_fractional)
		};

		for _ in 0..(long_vec.len() - short_vec.len()) {
			short_vec.push(0);
		}

		// both vec have the same length, we compare
		long_vec.iter().zip(short_vec.iter()).all(|(l_v, s_v)| l_v == s_v)
	}
}
