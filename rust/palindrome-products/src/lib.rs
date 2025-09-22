use std::{cmp, collections::HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
	value: u64,
	min_factor: u64,
	max_factor: u64,
}

impl Palindrome {
	pub fn new(value: u64, min_factor: u64, max_factor: u64) -> Self {
		if min_factor > max_factor {
			panic!("min_factor shouldn't be greater than max_factor");
		}

		Self { value, min_factor, max_factor }
	}

	pub fn value(&self) -> u64 {
		self.value
	}

	pub fn into_factors(self) -> HashSet<(u64, u64)> {
		let mut sols = HashSet::new();

		// try from 1 to the sqrt of the number
		let sqrt = (self.value as f64).sqrt() as u64;
		let upper_bound = cmp::min(cmp::max(sqrt, self.min_factor), self.max_factor);

		for i in self.min_factor..=upper_bound {
			if self.value % i == 0 {
				let quotient: u64 = self.value / i;
				if quotient >= self.min_factor && quotient <= self.max_factor {
					sols.insert((i, quotient));
				}
			}
		}

		sols
	}
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
	let is_palindrome = |num: u64| -> bool {
		let num_str = num.to_string();
		let reversed = num_str.chars().rev().collect::<String>();

		num_str == reversed
	};

	if min > max {
		return None;
	}

	let mut palindromes = HashSet::<u64>::new();

	for i in min..=max {
		for j in i..=max {
			let product = i * j;
			if is_palindrome(product) {
				palindromes.insert(product);
			}
		}
	}

	// convert from HashSet to vector and sort
	let mut palindromes = palindromes.drain().collect::<Vec<_>>();
	palindromes.sort();

	if !palindromes.is_empty() {
		Some((
			Palindrome::new(*palindromes.first().unwrap(), min, max),
			Palindrome::new(*palindromes.iter().last().unwrap(), min, max),
		))
	} else {
		None
	}
}
