fn is_prime(n: u32) -> bool {
	if n < 2 {
		return false;
	}
	!(2..=n.isqrt()).any(|i| n % i == 0)
}

pub fn nth(n: u32) -> u32 {
	let next_prime = |i: u32| -> u32 {
		let mut t = i;
		loop {
			if is_prime(t) {
				break;
			}
			t += 1;
		}
		t
	};

	let mut primes = vec![2];

	while n != (primes.len() as u32) - 1 {
		primes.push(next_prime(primes.last().unwrap() + 1));
	}

	*primes.last().unwrap()
}
