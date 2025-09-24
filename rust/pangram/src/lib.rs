/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
	let offset = b'a';
	let lower_cased = sentence.to_ascii_lowercase();

	let appeared =
		lower_cased.chars().filter(|c| c.is_ascii_lowercase()).fold([false; 26], |mut acc, c| {
			let idx = (c as u8 - offset) as usize;
			acc[idx] = true;
			acc
		});

	appeared.iter().all(|v| *v)
}
