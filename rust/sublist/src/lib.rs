#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
	Equal,
	Sublist,
	Superlist,
	Unequal,
}

#[rustfmt::skip]
pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
	// Check the length. get the `long_list`, and `short_list
	let mut switched = false;

	let [long_list, short_list] = if first_list.len() >= second_list.len() {
		[first_list, second_list]
	} else {
		switched = true;
		[second_list, first_list]
	};

	for idx in 0..=long_list.len() - short_list.len() {
		let extracted = &long_list[idx..idx + short_list.len()];

		match extracted == short_list {
			true if short_list.len() == long_list.len() => { return Comparison::Equal; },
			true => { return if switched { Comparison::Sublist } else { Comparison::Superlist }; },
			_ => {}
		}
	}

	Comparison::Unequal
}
