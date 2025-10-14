pub struct Matrix {
	m: Vec<Vec<u32>>,
}

impl Matrix {
	pub fn new(input: &str) -> Self {
		let m = input.split('\n').fold(vec![], |mut acc, row| {
			let row_vec: Vec<_> =
				row.split(char::is_whitespace).map(|tok| tok.parse::<u32>().unwrap()).collect();

			acc.push(row_vec);
			acc
		});

		Self { m }
	}

	pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
		if row_no < 1 || row_no > self.m.len() {
			return None;
		}

		Some(self.m[row_no - 1].clone())
	}

	pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
		let first_row = &self.m[0];
		if col_no < 1 || col_no > first_row.len() {
			return None;
		}

		let res: Vec<_> = self.m.iter().map(|row| row[col_no - 1]).clone().collect();

		Some(res)
	}
}
