pub struct RailFence {
	rails: u32,
}

pub enum State {
	Up,
	Down,
}

impl RailFence {
	pub fn new(rails: u32) -> RailFence {
		Self { rails }
	}

	pub fn encode(&self, text: &str) -> String {
		let mut blocks: Vec<String> = vec!["".to_string(); self.rails as usize];
		let mut write_row = 0;
		let mut state = State::Down;

		for c in text.chars() {
			// write a single col
			for r in 0..self.rails {
				blocks[r as usize].push(if r == write_row { c } else { ' ' });
			}
			(write_row, state) = self.transition_row_state(write_row, state);
		}

		blocks
			.into_iter()
			.map(|row_str| row_str.chars().filter(|c| c.is_alphanumeric()).collect::<String>())
			.collect::<Vec<_>>()
			.join("")
	}

	pub fn decode(&self, cipher: &str) -> String {
		let mut fillers: Vec<String> = vec!["".to_string(); self.rails as usize];
		let mut write_row = 0;
		let mut state = State::Down;

		// to build the structure for inserting the char in the rails
		for _ in cipher.chars() {
			// write a single col
			for r in 0..self.rails {
				fillers[r as usize].push(if r == write_row { '.' } else { ' ' });
			}
			(write_row, state) = self.transition_row_state(write_row, state);
		}

		let mut blocks: Vec<String> = vec![];
		let mut offset = 0;

		for filler in fillers {
			let mut row = "".to_string();
			for c in filler.chars() {
				if c == ' ' {
					row.push(' ');
				} else {
					row.push(cipher.chars().nth(offset).unwrap());
					offset += 1;
				}
			}
			blocks.push(row);
		}

		// Now we can read of the text from blocks;
		let mut res = "".to_string();
		offset = 0;
		let mut read_row = 0_u32;
		state = State::Down;

		while offset < cipher.len() {
			res.push(blocks[read_row as usize].chars().nth(offset).unwrap());
			(read_row, state) = self.transition_row_state(read_row, state);
			offset += 1;
		}

		res
	}

	fn transition_row_state(&self, row: u32, state: State) -> (u32, State) {
		match state {
			State::Up if row == 0 => (row + 1, State::Down),
			State::Up => (row - 1, State::Up),
			State::Down if row == self.rails - 1 => (row - 1, State::Up),
			State::Down => (row + 1, State::Down),
		}
	}
}
