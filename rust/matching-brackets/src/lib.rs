pub fn brackets_are_balanced(string: &str) -> bool {
	let mut stack: Vec<char> = Vec::new();

	for c in string.chars() {
		match c {
			'(' | '[' | '{' => stack.push(c),
			')' | ']' | '}' => {
				let open = match c {
					')' => '(',
					']' => '[',
					'}' => '{',
					_ => panic!("should not reach here"),
				};
				if stack.pop() != Some(open) {
					return false;
				}
			}
			_ => continue,
		}
	}
	stack.is_empty()
}
