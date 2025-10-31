use std::{
	cmp::{Ordering, PartialEq, PartialOrd},
	iter,
	ops::{Add, Mul, Neg, Sub},
};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone)]
pub struct Decimal {
	positive: bool,
	digits: Vec<u8>,
	decimal: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
	IntegralLenTooShort,
	FractionalLenTooShort,
}

impl Decimal {
	pub fn try_from(input: &str) -> Option<Decimal> {
		let mut digits = Vec::new();
		let mut decimal: Option<usize> = None;
		let mut positive = true;
		let mut offset = 0;

		for (idx, chr) in input.chars().enumerate() {
			if idx == 0 && chr == '-' {
				positive = false;
				offset = 1;
				continue;
			} else if idx == 0 && chr == '+' {
				offset = 1;
				continue;
			}

			match chr {
				'0'..='9' => {
					digits.push(chr as u8 - b'0');
				}
				'.' => {
					decimal = Some(idx - offset);
				}
				_ => {
					return None;
				}
			}
		}

		Some(Decimal { positive, digits, decimal })
	}

	fn integral_len(&self) -> usize {
		let Some(decimal) = self.decimal else {
			return self.digits.len();
		};

		decimal
	}

	fn integral(&self, expand_to: Option<usize>) -> Result<Vec<u8>, Error> {
		let decimal_or_digit_len = self.decimal.unwrap_or(self.digits.len());
		let mut res = self.digits[0..decimal_or_digit_len].to_vec();

		if let Some(expand_len) = expand_to {
			if expand_len < res.len() {
				return Err(Error::IntegralLenTooShort);
			}

			res = iter::repeat_n(0, expand_len - res.len()).chain(res).collect();
		}

		Ok(res)
	}

	fn fractional_len(&self) -> usize {
		let Some(decimal) = self.decimal else {
			return 0;
		};

		self.digits.len() - decimal
	}

	fn fractional(&self, expand_to: Option<usize>) -> Result<Vec<u8>, Error> {
		let mut fractional = match self.decimal {
			Some(decimal) if decimal >= self.digits.len() => Vec::new(),
			Some(decimal) => self.digits[decimal..].to_vec(),
			None => Vec::new(),
		};

		if let Some(expand_len) = expand_to {
			if expand_len < fractional.len() {
				return Err(Error::FractionalLenTooShort);
			}

			fractional.resize(expand_len, 0);
		}

		Ok(fractional)
	}

	// Helper functions
	fn reverse_ordering(ordering: Ordering, b_reverse: bool) -> Ordering {
		if !b_reverse {
			return ordering;
		}

		match ordering {
			Ordering::Less => Ordering::Greater,
			Ordering::Equal => Ordering::Equal,
			Ordering::Greater => Ordering::Less,
		}
	}

	fn inner_add_same_sign(&self, other: Self) -> Self {
		// get the fractional part
		let max_frac_len = self.fractional_len().max(other.fractional_len());
		let self_frac = self.fractional(Some(max_frac_len)).unwrap();
		let other_frac = other.fractional(Some(max_frac_len)).unwrap();
		let mut sum_frac = Vec::new();

		let mut carry = 0;
		for i in (0..max_frac_len).rev() {
			let mut res = self_frac[i] + other_frac[i] + carry;
			if res >= 10 {
				carry = 1;
				res -= 10;
			} else {
				carry = 0;
			}
			sum_frac.insert(0, res);
		}

		// get the integral part
		let max_integral_len = self.integral_len().max(other.integral_len());
		let self_integral = self.integral(Some(max_integral_len)).unwrap();
		let other_integral = other.integral(Some(max_integral_len)).unwrap();
		let mut sum_integral = Vec::new();

		// The last carry from fractional part is carried over
		for i in (0..max_integral_len).rev() {
			let mut res = self_integral[i] + other_integral[i] + carry;
			if res >= 10 {
				carry = 1;
				res -= 10;
			} else {
				carry = 0;
			}
			sum_integral.insert(0, res);
		}

		// handling the last carry, increase 1 digit value
		if carry == 1 {
			sum_integral.insert(0, 1);
		}

		Self {
			positive: self.positive,
			digits: sum_integral.iter().chain(sum_frac.iter()).cloned().collect::<Vec<_>>(),
			decimal: if sum_frac.is_empty() { None } else { Some(sum_integral.len()) },
		}
	}

	fn abs(&self) -> Self {
		let Decimal { positive: _, digits, decimal } = self;
		Self { positive: true, digits: digits.clone(), decimal: *decimal }
	}

	fn inner_sub(self, other: Self) -> Self {
		// invariant:
		// self is always greater than `other` in its magnitude, so the sign is always following
		//   the sign of self value. We are just substracting the magnitude of other from self.

		// get the fractional part
		let max_frac_len = self.fractional_len().max(other.fractional_len());
		let self_frac = self.fractional(Some(max_frac_len)).unwrap();
		let other_frac = other.fractional(Some(max_frac_len)).unwrap();
		let mut sum_frac = Vec::new();

		let mut carry: i8 = 0;
		for i in (0..max_frac_len).rev() {
			let mut res = self_frac[i] as i8 - other_frac[i] as i8 - carry;
			if res < 0 {
				carry = 1;
				res += 10;
			} else {
				carry = 0;
			}
			sum_frac.insert(0, res as u8);
		}

		// get the integral part
		let max_integral_len = self.integral_len().max(other.integral_len());
		let self_integral = self.integral(Some(max_integral_len)).unwrap();
		let other_integral = other.integral(Some(max_integral_len)).unwrap();
		let mut sum_integral = Vec::new();

		// The last carry from fractional part is carried over
		for i in (0..max_integral_len).rev() {
			let mut res = self_integral[i] as i8 - other_integral[i] as i8 - carry;
			if res < 0 {
				carry = 1;
				res += 10;
			} else {
				carry = 0;
			}
			sum_integral.insert(0, res as u8);
		}

		// note: there is no way there is a last carry because of the invariant
		//   that self should be >= others
		if carry > 0 {
			panic!("there should be no carry at the end of inner_sub");
		}

		Self {
			positive: self.positive,
			digits: sum_integral.iter().chain(sum_frac.iter()).cloned().collect::<Vec<_>>(),
			decimal: if sum_frac.is_empty() { None } else { Some(sum_integral.len()) },
		}
	}

	fn inner_digits_add(one: &[u8], another: &[u8]) -> Vec<u8> {
		// Ensure both value have the same number of digits
		let max_len = one.len().max(another.len());
		let one: Vec<_> = iter::repeat_n(0, max_len - one.len()).chain(one.to_vec()).collect();
		let another: Vec<_> =
			iter::repeat_n(0, max_len - another.len()).chain(another.to_vec()).collect();
		let mut res = Vec::new();

		let mut carry = 0;
		for digit_idx in (0..one.len()).rev() {
			let mut val = one[digit_idx] + another[digit_idx] + carry;
			if val >= 10 {
				carry = 1;
				val -= 10;
			} else {
				carry = 0;
			}
			res.insert(0, val);
		}

		if carry == 1 {
			res.insert(0, 1);
		}

		res
	}

	fn inner_single_digit_mul(digits: &[u8], single: &u8) -> Vec<u8> {
		if *single == 0 {
			return Vec::new();
		}
		let digits = digits.to_vec();
		let mut res = digits.clone();

		for _ in 1..*single {
			res = Self::inner_digits_add(&res, &digits);
		}

		res
	}
}

impl PartialEq for Decimal {
	fn eq(&self, other: &Decimal) -> bool {
		// algorithm
		if self.positive != other.positive {
			return false;
		}

		// check the integer part
		let max_int_len = self.integral_len().max(other.integral_len());
		let self_int = self.integral(Some(max_int_len)).unwrap();
		let other_int = other.integral(Some(max_int_len)).unwrap();

		if self_int != other_int {
			return false;
		}

		let max_frac_len = self.fractional_len().max(other.fractional_len());
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
		let max_integral_len = self.integral_len().max(other.integral_len());
		let self_integral = self.integral(Some(max_integral_len)).unwrap();
		let other_integral = other.integral(Some(max_integral_len)).unwrap();

		for (s_v, o_v) in self_integral.iter().zip(other_integral.iter()) {
			if s_v != o_v {
				return s_v.partial_cmp(o_v).map(|o| Decimal::reverse_ordering(o, !self.positive));
			}
		}

		// check the fractional part
		let max_frac_len = self.fractional_len().max(other.fractional_len());
		let self_frac = self.fractional(Some(max_frac_len)).unwrap();
		let other_frac = other.fractional(Some(max_frac_len)).unwrap();

		for (s_v, o_v) in self_frac.iter().zip(other_frac.iter()) {
			if s_v != o_v {
				return s_v.partial_cmp(o_v).map(|o| Decimal::reverse_ordering(o, !self.positive));
			}
		}

		Some(Ordering::Equal)
	}
}

impl Add for Decimal {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let sign_equal = self.positive == rhs.positive;

		if sign_equal {
			self.inner_add_same_sign(rhs)
		} else {
			let self_abs = self.abs();
			let rhs_abs = rhs.abs();

			if self_abs >= rhs_abs { self.inner_sub(rhs_abs) } else { rhs.inner_sub(self_abs) }
		}
	}
}

impl Sub for Decimal {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		self.add(rhs.neg())
	}
}

impl Neg for Decimal {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self { positive: !self.positive, digits: self.digits, decimal: self.decimal }
	}
}

impl Mul for Decimal {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let positive = self.positive == rhs.positive;

		let mut res: Vec<_> = vec![];
		for (idx, digit) in self.digits.iter().enumerate() {
			let mut inter = Self::inner_single_digit_mul(&rhs.digits, digit);
			inter.resize(inter.len() + self.digits.len() - idx - 1, 0);
			res = Self::inner_digits_add(&res, &inter);
		}

		// decimal
		let decimal = res.len() - self.fractional_len() - rhs.fractional_len();

		Self { positive, digits: res, decimal: Some(decimal) }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Create a Decimal from a string literal
	///
	/// Use only when you _know_ that your value is valid.
	fn decimal(input: &str) -> Decimal {
		Decimal::try_from(input).expect("That was supposed to be a valid value")
	}

	#[test]
	fn basic1() {
		let res = decimal("10.015");
		assert_eq!(res.integral_len(), 2);
		assert_eq!(res.integral(None).unwrap(), vec![1, 0]);
		assert_eq!(res.integral(Some(4)).unwrap(), vec![0, 0, 1, 0]);

		assert_eq!(res.fractional_len(), 3);
		assert_eq!(res.fractional(None).unwrap(), vec![0, 1, 5]);
		assert_eq!(res.fractional(Some(5)).unwrap(), vec![0, 1, 5, 0, 0]);

		assert!(res.fractional(Some(2)).is_err());
	}

	#[test]
	fn basic2() {
		let res = decimal("1.0");
		assert_eq!(res.digits, vec![1, 0]);
		assert_eq!(res.decimal.unwrap(), 1);

		assert_eq!(res.integral_len(), 1);
		assert_eq!(res.integral(None).unwrap(), vec![1]);
		assert_eq!(res.fractional_len(), 1);
		assert_eq!(res.fractional(None).unwrap(), vec![0]);
	}

	#[test]
	fn basic3() {
		let res = decimal("1");
		assert_eq!(res.digits, vec![1]);
		assert!(res.decimal.is_none());

		assert_eq!(res.integral_len(), 1);
		assert_eq!(res.integral(None).unwrap(), vec![1]);
		assert_eq!(res.fractional_len(), 0);
		assert_eq!(res.fractional(None).unwrap(), vec![]);
	}

	#[test]
	fn basic4() {
		let res = decimal("1.");
		assert_eq!(res.digits, vec![1]);
		assert_eq!(res.decimal.unwrap(), 1);

		assert_eq!(res.integral_len(), 1);
		assert_eq!(res.integral(None).unwrap(), vec![1]);
		assert_eq!(res.fractional_len(), 0);
		assert_eq!(res.fractional(None).unwrap(), vec![]);
	}

	#[test]
	fn basic5() {
		let res = decimal(".1");
		assert_eq!(res.digits, vec![1]);
		assert_eq!(res.decimal.unwrap(), 0);

		assert_eq!(res.integral_len(), 0);
		assert_eq!(res.integral(None).unwrap(), vec![]);
		assert_eq!(res.fractional_len(), 1);
		assert_eq!(res.fractional(None).unwrap(), vec![1]);
	}

	#[test]
	fn basic6() {
		let res = decimal("0.1");
		assert_eq!(res.digits, vec![0, 1]);
		assert_eq!(res.decimal.unwrap(), 1);

		assert_eq!(res.integral_len(), 1);
		assert_eq!(res.integral(None).unwrap(), vec![0]);
		assert_eq!(res.fractional_len(), 1);
		assert_eq!(res.fractional(None).unwrap(), vec![1]);
	}

	#[test]
	fn basic7() {
		let res = decimal("-0.1");
		assert_eq!(res.digits, vec![0, 1]);
		assert_eq!(res.decimal.unwrap(), 1);
		assert_eq!(res.positive, false);

		assert_eq!(res.integral_len(), 1);
		assert_eq!(res.integral(None).unwrap(), vec![0]);
		assert_eq!(res.fractional_len(), 1);
		assert_eq!(res.fractional(None).unwrap(), vec![1]);
	}
}
