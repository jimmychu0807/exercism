use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
	let mut map: HashMap<String, u32> = HashMap::new();

	let trim_front = |word: String| -> String {
		let mut offset = 0;
		while offset < word.len() {
			if word.chars().nth(offset).unwrap().is_alphanumeric() {
				break;
			}
			offset += 1;
		}
		word[offset..].to_string()
	};

	let clean_word = |word: String| -> Option<String> {
		let mut cleaned: String = trim_front(word).chars().rev().collect();
		cleaned = trim_front(cleaned).chars().rev().collect();

		if cleaned.is_empty() { None } else { Some(cleaned) }
	};

	words
		.split(|c: char| c.is_whitespace() || c == ',' || c == '.')
		.map(str::to_lowercase)
		.filter_map(clean_word)
		.for_each(|word| {
			map.entry(word).and_modify(|v| *v += 1).or_insert(1);
		});

	map
}
