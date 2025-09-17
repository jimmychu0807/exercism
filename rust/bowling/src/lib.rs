#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	NotEnoughPinsLeft,
	GameComplete,
}

const PINS_IN_FRAME: u16 = 10;

#[derive(Debug, PartialEq, Eq)]
pub struct Round(Option<u16>, Option<u16>);

impl Round {
	pub fn new(pins: u16) -> Self {
		Self(Some(pins), None)
	}

	pub fn is_striked(&self) -> bool {
		self.0 == Some(PINS_IN_FRAME)
	}

	pub fn is_spared(&self) -> bool {
		if self.0.is_none() || self.1.is_none() {
			return false;
		}
		self.0.unwrap() + self.1.unwrap() == PINS_IN_FRAME
	}

	pub fn is_completed(&self) -> bool {
		if self.is_striked() || self.is_spared() {
			return true;
		}
		self.0.is_some() && self.1.is_some()
	}

	pub fn remaining_pins(&self) -> u16 {
		if self.is_completed() {
			return 0;
		}
		PINS_IN_FRAME - self.0.unwrap_or(0) - self.1.unwrap_or(0)
	}
}

pub struct BowlingGame {
	frames: Vec<Round>,
}

impl BowlingGame {
	pub fn new() -> Self {
		Self { frames: vec![] }
	}

	pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
		if self.frames.len() == 10 {
			let tenth_frame = self.frames.get(9).unwrap();
			if tenth_frame.is_completed() && !tenth_frame.is_striked() && !tenth_frame.is_spared() {
				return Err(Error::GameComplete);
			}
		} else if self.frames.len() == 11 {
			let tenth_frame = self.frames.get(9).unwrap();
			let eleventh_frame = self.frames.get(10).unwrap();

			if !tenth_frame.is_striked() {
				// The player has made one extra roll already, thus the 11th round. So the game has
				// completed.
				return Err(Error::GameComplete);
			}

			if eleventh_frame.is_completed() && !eleventh_frame.is_striked() {
				return Err(Error::GameComplete);
			}
		} else if self.frames.len() > 11 {
			return Err(Error::GameComplete);
		}

		let remaining_pins = if self.frames.is_empty() {
			PINS_IN_FRAME
		} else {
			let last_frame = self.frames.iter().last().unwrap();
			if last_frame.is_completed() { PINS_IN_FRAME } else { last_frame.remaining_pins() }
		};

		if pins > remaining_pins {
			return Err(Error::NotEnoughPinsLeft);
		}

		// insert the pins value
		if self.frames.is_empty() {
			self.frames.push(Round::new(pins));
		} else {
			let last_frame = self.frames.iter_mut().last().unwrap();
			if last_frame.is_completed() {
				self.frames.push(Round::new(pins));
			} else {
				last_frame.1 = Some(pins);
			}
		}

		Ok(())
	}

	pub fn score(&self) -> Option<u16> {
		// Incomplete game cannot be scored. Determine if a game has completed
		if self.frames.len() < 10 {
			return None;
		} else {
			let round_len = self.frames.len();
			let tenth_frame = self.frames.get(9).unwrap();

			// if the tenth frame is not completed, the whole game is not completed
			if !tenth_frame.is_completed() {
				return None;
			}

			// if the tenth frame is spared, has one more fill ball, so game extends to 11 rounds
			if tenth_frame.is_spared() && round_len < 11 {
				return None;
			}

			// if the tenth frame is striked, need further logics
			if tenth_frame.is_striked() {
				if round_len < 11 {
					return None;
				}
				let eleventh_frame = self.frames.get(10).unwrap();
				if eleventh_frame.is_striked() && round_len < 12 {
					return None;
				}
				if !eleventh_frame.is_completed() {
					return None;
				}
			}
		}

		let score = (0..10).fold(0, |acc, idx| {
			let round = self.frames.get(idx).unwrap();
			if round.is_striked() {
				// get next throw
				let next_frame = self.frames.get(idx + 1).unwrap();
				let next_throw = next_frame.0.unwrap();

				// get next next throw
				let next_next_throw = if !next_frame.is_striked() {
					next_frame.1.unwrap()
				} else {
					let next_next_frame = self.frames.get(idx + 2).unwrap();
					next_next_frame.0.unwrap()
				};

				acc + PINS_IN_FRAME + next_throw + next_next_throw
			} else if round.is_spared() {
				// get next throw
				let next_frame = self.frames.get(idx + 1).unwrap();
				acc + PINS_IN_FRAME + next_frame.0.unwrap()
			} else {
				acc + round.0.unwrap() + round.1.unwrap()
			}
		});

		Some(score)
	}
}
