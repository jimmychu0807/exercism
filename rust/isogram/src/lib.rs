use std::collections::HashMap;

const FILTERED_CHARS: [char; 2] = ['-', ' '];

pub fn check(candidate: &str) -> bool {
	let mut char_map: HashMap<char, bool> = HashMap::new();

	let mut cand_string = String::from(candidate);
	cand_string.make_ascii_lowercase();
	cand_string = cand_string.chars().filter(|c| !FILTERED_CHARS.contains(c)).collect::<String>();

	for chr in cand_string.chars() {
		if char_map.contains_key(&chr) {
			return false;
		}
		char_map.insert(chr, true);
	}

	true
}
