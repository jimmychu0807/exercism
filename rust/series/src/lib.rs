pub fn series(digits: &str, len: usize) -> Vec<String> {
	let chars = digits.chars().collect::<Vec<_>>();

	chars.windows(len).map(|chars| chars.iter().collect::<String>()).collect::<Vec<String>>()
}
