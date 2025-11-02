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

	let remaining = input[1..].to_vec();
	let target = input[0];
	let sol = rec_find(&target, &remaining);

	let mut chain = sol?;
	chain.insert(0, target);
	if chain[0].0 == chain.last().unwrap().1 {
		return Some(chain);
	}

	None
}

fn rec_find(current: &(u8, u8), remaining: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
	// terminal condition: no more in remaining, this is good
	if remaining.is_empty() {
		return Some(Vec::new());
	};

	// usual condition
	let current_val = current.1;
	let filtered: Vec<_> = remaining
		.iter()
		.enumerate()
		.filter(|(_, (l, r))| current_val == *l || current_val == *r)
		.collect();

	if filtered.is_empty() {
		return None;
	}

	for (idx, focus) in filtered {
		let mut focus = *focus;
		// re-orient `focus` domino if necessary
		if current_val != focus.0 {
			focus = rev_domino(focus);
		}

		// clone the remaining out, and take out `focus` from the remaining stack
		let mut remaining = remaining.to_vec();
		remaining.remove(idx);

		// trying out each one, call rec_find
		let sol = rec_find(&focus, &remaining);

		if let Some(mut current_chain) = sol {
			current_chain.insert(0, focus);
			return Some(current_chain);
		};
	}

	None
}

fn rev_domino(domino: (u8, u8)) -> (u8, u8) {
	(domino.1, domino.0)
}
