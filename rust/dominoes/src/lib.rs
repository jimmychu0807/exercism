pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
	if input.is_empty() {
		return Some(vec![]);
	}
	if input.len() == 1 {
		let (l, r) = input[0];
		return match l == r {
			true => Some(input.to_vec()),
			false => None,
		};
	}

	Some(vec![])
}
