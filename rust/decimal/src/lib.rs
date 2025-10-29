use std::cmp::{Ordering, PartialEq, PartialOrd};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug)]
pub struct Decimal {
	positive: bool,
	digits: Vec<u8>,
	decimal: usize,
}

#[derive(Debug)]
pub enum Error {
	IntegralLenTooShort,
	FractionalLenTooShort,
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

	pub fn integral(&self, expand_to: Option<usize>) -> Result<Vec<u8>, Error> {
		let mut res = self.digits[0..self.decimal].to_vec();

		if let Some(expand_len) = expand_to {
			if expand_len < res.len() {
				return Err(Error::IntegralLenTooShort);
			}

			for _ in 0..expand_len - res.len() {
				res.insert(0, 0);
			}
		}

		Ok(res)
	}

	pub fn fractional_digits(&self) -> usize {
		if self.decimal < self.digits.len() - 1 { self.digits.len() - self.decimal } else { 0 }
	}

	pub fn fractional(&self, expand_to: Option<usize>) -> Result<Vec<u8>, Error> {
		let mut fractional = if self.decimal < self.digits.len() - 1 {
			self.digits[self.decimal + 1..].to_vec()
		} else {
			Vec::new()
		};

		if let Some(expand_len) = expand_to {
			if expand_len < fractional.len() {
				return Err(Error::FractionalLenTooShort);
			}

			fractional.resize(expand_len, 0);
		}

		Ok(fractional)
	}
}

impl PartialEq for Decimal {
	fn eq(&self, other: &Decimal) -> bool {
		// algorithm
		if self.positive != other.positive {
			return false;
		}

		// check the integer part
		let max_integral_len = self.decimal.max(other.decimal);
		if self.integral(Some(max_integral_len)).unwrap()
			!= other.integral(Some(max_integral_len)).unwrap()
		{
			return false;
		}

		let max_frac_len = self.fractional_digits().max(other.fractional_digits());
		let self_frac = self.fractional(Some(max_frac_len)).unwrap();
		let other_frac = other.fractional(Some(max_frac_len)).unwrap();

		// both vec have the same length, now we compare
		self_frac.iter().zip(other_frac.iter()).all(|(l_v, s_v)| l_v == s_v)
	}
}

impl PartialOrd for Decimal {
	fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
		// check sign first
		if self.positive != other.positive {
			if self.positive {
				return Some(Ordering::Greater);
			} else {
				return Some(Ordering::Less);
			}
		};

		// check the integral part
		let max_integral_len = self.decimal.max(other.decimal);
		let self_integral = self.integral(Some(max_integral_len)).unwrap();
		let other_integral = other.integral(Some(max_integral_len)).unwrap();
		for (s_v, o_v) in self_integral.iter().zip(other_integral.iter()) {
			if s_v != o_v {
				if self.positive {
					return s_v.partial_cmp(o_v);
				} else {
					// this is a negative value, revert the partial_cmp result
					if s_v > o_v {
						return Some(Ordering::Less);
					} else {
						return Some(Ordering::Greater);
					}
				}
			}
		}

		// check the fractional part
		let max_frac_len = self.fractional_digits().max(other.fractional_digits());
		let self_frac = self.fractional(Some(max_frac_len)).unwrap();
		let other_frac = other.fractional(Some(max_frac_len)).unwrap();
		for (s_v, o_v) in self_frac.iter().zip(other_frac.iter()) {
			if s_v != o_v {
				if self.positive {
					return s_v.partial_cmp(o_v);
				} else {
					// this is a negative value, revert the partial_cmp result
					if s_v > o_v {
						return Some(Ordering::Less);
					} else {
						return Some(Ordering::Greater);
					}
				}
			}
		}

		Some(Ordering::Equal)
	}
}
