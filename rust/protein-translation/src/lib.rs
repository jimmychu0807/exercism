use std::collections::HashMap;

pub fn translate(dna: &str) -> Option<Vec<&str>> {
	let mut sol: Vec<&str> = vec![];
	let map: HashMap<&str, &str> = [
		("AUG", "Methionine"),
		("UUU", "Phenylalanine"),
		("UUC", "Phenylalanine"),
		("UUA", "Leucine"),
		("UUG", "Leucine"),
		("UCU", "Serine"),
		("UCC", "Serine"),
		("UCA", "Serine"),
		("UCG", "Serine"),
		("UAU", "Tyrosine"),
		("UAC", "Tyrosine"),
		("UGU", "Cysteine"),
		("UGC", "Cysteine"),
		("UGG", "Tryptophan"),
	]
	.into_iter()
	.collect();

	let stop: Vec<&str> = ["UAA", "UAG", "UGA"].into_iter().collect();

	let mut start_offset = 0;
	while start_offset < dna.len() {
		let end_offset = dna.len().min(start_offset + 3);
		let slice = &dna[start_offset..end_offset];

		if stop.contains(&slice) {
			break;
		}

		match map.get(slice) {
			Some(data) => {
				sol.push(data);
			}
			None => return None,
		}

		start_offset = end_offset;
	}

	Some(sol)
}
