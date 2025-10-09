enum Op {
	Set,
	Add,
	Minus,
	Multiply,
	Divide,
	Power,
}

#[derive(PartialEq)]
enum State {
	ReadVal,
	ReadOp,
}

pub fn answer(command: &str) -> Option<i32> {
	let mut acc: Option<i32> = None;
	let mut op: Op = Op::Set;
	let mut state = State::ReadVal;

	let mut command = command;
	// rm the last question mark
	if command.ends_with('?') {
		command = &command[..command.len() - 1];
	}

	for (_, token) in command.split(' ').enumerate().filter(|(idx, _)| idx > &1_usize) {
		match state {
			State::ReadVal => {
				if let Some(val) = parse_token_to_val(token) {
					match op {
						Op::Set => {
							acc = Some(val);
						}
						Op::Add => {
							acc = acc.map(|acc| acc + val);
						}
						Op::Minus => {
							acc = acc.map(|acc| acc - val);
						}
						Op::Multiply => {
							acc = acc.map(|acc| acc * val);
						}
						Op::Divide => {
							acc = acc.map(|acc| acc / val);
						}
						Op::Power => {
							acc = acc.map(|acc| acc.pow(val as u32));
						}
					}

					state = State::ReadOp;
				} else {
					return None;
				}
			}

			State::ReadOp => match token {
				"plus" => {
					op = Op::Add;
					state = State::ReadVal;
				}
				"minus" => {
					op = Op::Minus;
					state = State::ReadVal;
				}
				"multiplied" => {
					op = Op::Multiply;
				}
				"divided" => {
					op = Op::Divide;
				}
				"raised" => {
					op = Op::Power;
				}
				"to" | "power" => {}
				"by" | "the" => {
					state = State::ReadVal;
				}
				_ => {
					return None;
				}
			},
		}
	}

	if state == State::ReadVal {
		return None;
	}

	acc
}

fn parse_token_to_val(token: &str) -> Option<i32> {
	if let Ok(val) = token.parse::<i32>() {
		return Some(val);
	}

	if token.len() >= 3 {
		if let Ok(val) = &token[0..token.len() - 2].parse::<i32>() {
			return Some(*val);
		}
	}

	None
}
