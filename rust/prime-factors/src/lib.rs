fn is_prime(n: u64) -> bool {
	if n < 2 {
		return false;
	}
	!(2..=n.isqrt()).any(|i| n % i == 0)
}

pub fn factors(n: u64) -> Vec<u64> {
	let mut pfactors: Vec<u64> = vec![];
	let prime_candidates = (2..=n.isqrt()).filter(|n| is_prime(*n)).collect::<Vec<u64>>();

	let mut remaining = n;
	loop {
		if remaining == 1 {
			break;
		}
		if let Some(factor) =
			prime_candidates.clone().into_iter().find(|c| remaining.is_multiple_of(*c))
		{
			pfactors.push(factor);
			remaining /= factor;
			continue;
		}
		pfactors.push(remaining);
		remaining = 1;
	}

	pfactors
}
