pub fn square(s: u32) -> u64 {
	if !(1..=64).contains(&s) {
		panic!("s should be between 1 - 64");
	}
	1_u64 << (s - 1)
}

pub fn total() -> u64 {
	(1..=64).map(square).sum::<u64>()
}
