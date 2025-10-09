// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum Direction {
	North,
	East,
	South,
	West,
}

#[derive(Clone)]
pub struct Robot {
	pos_x: i32,
	pos_y: i32,
	facing: Direction,
}

impl Robot {
	pub fn new(x: i32, y: i32, d: Direction) -> Self {
		Self { pos_x: x, pos_y: y, facing: d }
	}

	#[must_use]
	pub fn turn_right(self) -> Self {
		let mut ret = self.clone();

		match self.facing {
			Direction::North => ret.facing = Direction::East,
			Direction::East => ret.facing = Direction::South,
			Direction::South => ret.facing = Direction::West,
			Direction::West => ret.facing = Direction::North,
		}

		ret
	}

	#[must_use]
	pub fn turn_left(self) -> Self {
		let mut ret = self.clone();

		match self.facing {
			Direction::North => ret.facing = Direction::West,
			Direction::West => ret.facing = Direction::South,
			Direction::South => ret.facing = Direction::East,
			Direction::East => ret.facing = Direction::North,
		}

		ret
	}

	#[must_use]
	pub fn advance(self) -> Self {
		let mut ret = self.clone();

		match self.facing {
			Direction::North => ret.pos_y += 1,
			Direction::East => ret.pos_x += 1,
			Direction::South => ret.pos_y -= 1,
			Direction::West => ret.pos_x -= 1,
		}

		ret
	}

	#[must_use]
	pub fn instructions(self, instructions: &str) -> Self {
		instructions.chars().fold(self, |robot, ins| match ins {
			'A' => robot.advance(),
			'L' => robot.turn_left(),
			'R' => robot.turn_right(),
			_ => panic!("shouldn't reach here"),
		})
	}

	pub fn position(&self) -> (i32, i32) {
		(self.pos_x, self.pos_y)
	}

	pub fn direction(&self) -> &Direction {
		&self.facing
	}
}
