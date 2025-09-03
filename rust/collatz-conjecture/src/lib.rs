pub fn collatz(n: u64) -> Option<u64> {
	if n < 1 {
		return None;
	}
	let mut steps = 0_u64;
	let mut current = n;
	while current > 1 {
		current = if current % 2 == 0 { current / 2 } else { 3 * current + 1 };
		steps += 1;
	}
	Some(steps)
}
