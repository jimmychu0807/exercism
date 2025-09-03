pub fn egg_count(display_value: u32) -> usize {
    // keep doing a right-shift on the number until it is 0
	let mut cnt = 0_usize;
	let mut current = display_value;

	while current > 0 {
		if current % 2 == 1 {
			cnt += 1;
		}
		current >>= 1;
	}

	cnt
}
