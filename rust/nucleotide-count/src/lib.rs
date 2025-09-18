use std::collections::{HashMap, hash_map::Entry};

const VALID_CHARS: &[char] = &['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
	let mut cnt: usize = 0;

	// check the nucleotide
	if !VALID_CHARS.contains(&nucleotide) {
		return Err(nucleotide);
	}

	for chr in dna.chars() {
		if !VALID_CHARS.contains(&chr) {
			return Err(chr);
		}
		if chr == nucleotide {
			cnt += 1;
		}
	}
	Ok(cnt)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
	let mut map: HashMap<char, usize> = HashMap::new();

	for chr in dna.chars() {
		if !VALID_CHARS.contains(&chr) {
			return Err(chr);
		}

		match map.entry(chr) {
			Entry::Occupied(mut e) => e.insert(e.get() + 1),
			Entry::Vacant(e) => *e.insert(1),
		};
	}

	// ensure the hashmap contains all A, C, G, T chars
	for chr in VALID_CHARS.iter() {
		if !map.contains_key(chr) {
			map.insert(*chr, 0);
		}
	}

	Ok(map)
}
