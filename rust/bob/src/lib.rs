pub fn reply(message: &str) -> &str {
	let trimmed = message.trim();

	if trimmed.is_empty() {
		return "Fine. Be that way!";
	}

	let all_caps = trimmed.chars().any(char::is_alphabetic) && trimmed == trimmed.to_uppercase();
	let is_question = trimmed.ends_with('?');
	match (all_caps, is_question) {
		(true, true) => "Calm down, I know what I'm doing!",
		(true, _) => "Whoa, chill out!",
		(_, true) => "Sure.",
		_ => "Whatever.",
	}
}
