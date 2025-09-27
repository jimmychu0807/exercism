use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
	Abundant,
	Perfect,
	Deficient,
}

fn is_prime(n: u64) -> bool {
	if n < 2 {
		return false;
	}
	!(2..=n.isqrt()).any(|i| n % i == 0)
}

fn next_prime(n: u64) -> u64 {
	let mut next_try = n + 1;
	loop {
		if is_prime(next_try) {
			return next_try;
		}
		next_try += 1;
	}
}

fn cross_multiply(factor: &HashSet<u64>, max: u64) -> HashSet<u64> {
	let second = factor.clone();
	let mut vec: Vec<u64> = factor.iter().cloned().collect::<Vec<_>>();

	for i in factor.iter() {
		for j in second.iter() {
			let product = i * j;
			if product < max && max % product == 0 {
				vec.push(product);
			}
		}
	}

	vec.into_iter().collect::<HashSet<_>>()
}

pub fn classify(num: u64) -> Option<Classification> {
	if num == 0 {
		return None;
	} else if num <= 2 || is_prime(num) {
		return Some(Classification::Deficient);
	}

	let mut factors = HashSet::new();

	let mut divisor = num;

	'outer: while divisor > 1 {
		let mut prime = 2;
		while divisor % prime != 0 {
			prime = next_prime(prime);

			if prime >= divisor {
				break 'outer;
			}
		}

		// divisor % prime == 0 by this point
		divisor /= prime;
		factors.insert(prime);

		if divisor > 1 {
			factors.insert(divisor);
		}
	}

	loop {
		let orig_size = factors.len();
		factors = cross_multiply(&factors, num);

		let new_size = factors.len();
		if orig_size == new_size {
			break;
		}
	}

	let sum = factors.into_iter().fold(1, |acc, v| acc + v);

	if sum == num {
		Some(Classification::Perfect)
	} else if sum > num {
		Some(Classification::Abundant)
	} else {
		Some(Classification::Deficient)
	}
}
