#[derive(Debug, PartialEq)]
enum Direction {
	Right,
	Down,
	Left,
	Up,
}

impl Direction {
	pub fn new() -> Self {
		Self::Right
	}

	pub fn next(self) -> Self {
		match self {
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
			Direction::Up => Direction::Right,
		}
	}
}

#[derive(Debug)]
struct Cell {
	row: usize,
	col: usize,
}

impl Cell {
	pub fn new() -> Self {
		Self { row: 0, col: 0 }
	}

	pub fn next(&mut self, dir: &Direction) {
		match dir {
			Direction::Right => {
				self.col += 1;
			}
			Direction::Down => {
				self.row += 1;
			}
			Direction::Left => {
				self.col -= 1;
			}
			Direction::Up => {
				self.row -= 1;
			}
		}
	}
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
	if size == 0 {
		return vec![];
	}

	let size_uz = size as usize;

	let mut sol = vec![vec![0; size_uz]; size_uz];
	let mut dir = Direction::new();
	let mut next_cell = Cell::new();
	let mut next_val = 1;

	while sol[next_cell.row][next_cell.col] == 0 {
		sol[next_cell.row][next_cell.col] = next_val;
		next_val += 1;

		if ((next_cell.col == size_uz - 1 || sol[next_cell.row][next_cell.col + 1] != 0)
			&& dir == Direction::Right)
			|| ((next_cell.row == size_uz - 1 || sol[next_cell.row + 1][next_cell.col] != 0)
				&& dir == Direction::Down)
			|| ((next_cell.col == 0 || sol[next_cell.row][next_cell.col - 1] != 0)
				&& dir == Direction::Left)
			|| ((next_cell.row == 0 || sol[next_cell.row - 1][next_cell.col] != 0)
				&& dir == Direction::Up)
		{
			dir = dir.next();
		}

		next_cell.next(&dir);

		if next_cell.row >= size_uz || next_cell.col >= size_uz {
			break;
		}
	}

	sol
}
