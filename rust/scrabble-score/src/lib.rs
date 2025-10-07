/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
	word.to_ascii_uppercase().chars().fold(0, |acc, chr| acc + char_score(&chr))
}

fn char_score(chr: &char) -> u64 {
	match chr {
		'D' | 'G' => 2,
		'B' | 'C' | 'M' | 'P' => 3,
		'F' | 'H' | 'V' | 'W' | 'Y' => 4,
		'K' => 5,
		'J' | 'X' => 8,
		'Q' | 'Z' => 10,
		'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
		_ => 0,
	}
}
