use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
	possible_anagrams
		.iter()
		.filter(|test_str| is_anagram(word, test_str))
		.copied()
		.collect::<HashSet<_>>()
}

fn is_anagram(word1: &str, word2: &str) -> bool {
	// a word is not its own anagram
	let [lower_word1, lower_word2] = [word1, word2].map(|word| word.to_lowercase());
	if lower_word1 == lower_word2 {
		return false;
	}

	let [vec1, vec2] = [lower_word1, lower_word2].map(|word| {
		let mut vec = word.chars().collect::<Vec<char>>();
		vec.sort();
		vec
	});

	vec1 == vec2
}
