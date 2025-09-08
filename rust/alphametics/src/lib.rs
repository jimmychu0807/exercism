use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
	// Algorithm:
	// 1. parse the line, build up the search space
	//   a hashMap of alphabet and a vector of u8, from 0 - 9
	// 2. build up LHS and RHS, by the split of "==".
	// 3. build up the expression tree and compute the value by a stack

	let search_map: HashMap<char, Vec<u8>> = build_search_map(input);
	let search_space: Vec<HashMap<char, u8>> = build_search_space(&search_map);
	// println!("search space len: {}", search_space.len());

	let mut solutions: Vec<HashMap<char, u8>> = vec![];

	let [left_exp, right_exp, ..] = input.split("==").map(str::trim).collect::<Vec<&str>>()[..]
	else {
		panic!("Not expecting for more than two parts")
	};

	for s in search_space {
		if parse_expression(left_exp, &s) == parse_expression(right_exp, &s) {
			solutions.push(s.clone());
			// non-unique value return None in the algo
			if solutions.len() > 1 { break; }
		}
	}

	match solutions.len() {
		1 => Some(solutions[0].clone()),
		_ => None,
	}
}

fn parse_expression(exp: &str, sol_map: &HashMap<char, u8>) -> u64 {
	let substitute_value = |en: &str, sol_map: &HashMap<char, u8>| -> u64 {
		en.chars().fold(0, |acc, c| acc * 10 + *sol_map.get(&c).unwrap_or(&0) as u64)
	};

	let entries = exp.split("+").map(str::trim).collect::<Vec<_>>();
	entries
		.into_iter()
		.map(|en| substitute_value(en, sol_map))
		.sum()
}

fn build_search_map(input: &str) -> HashMap<char, Vec<u8>> {
	let mut search_map = HashMap::new();
	let from_zero: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	let from_one: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

	for (idx, c) in input.chars().enumerate() {
		if !c.is_ascii_uppercase() {
			continue;
		}

		// The first digit of a value can't be a leading zero. Determine if this is the first digit
		if idx == 0 || !input.chars().nth(idx - 1).unwrap().is_ascii_uppercase() {
			search_map.insert(c, from_one.clone());
		} else {
			search_map.entry(c).or_insert_with(|| from_zero.clone());
		}
	}

	search_map
}

fn build_search_space(search_map: &HashMap<char, Vec<u8>>) -> Vec<HashMap<char, u8>> {
	let mut arr: VecDeque<HashMap<char, u8>> = VecDeque::new();
	arr.push_back(HashMap::new());

	for c in search_map.keys() {
		// all possible values that `c` can be
		let all_vals = search_map.get(c).unwrap();
		let to_pop = arr.len();
		for _ in 0..to_pop {
			let existing_map = arr.pop_front().unwrap();

			// remove the values used in existing_map from all_vals
			let existing_values = existing_map.values().collect::<Vec<_>>();
			let possible_vals = all_vals.iter().filter(|v| !existing_values.contains(v));

			for v in possible_vals {
				let mut new_map = existing_map.clone();
				new_map.insert(*c, *v);
				arr.push_back(new_map);
			}
		}
	}

	arr.into_iter().collect::<Vec<_>>()
}
