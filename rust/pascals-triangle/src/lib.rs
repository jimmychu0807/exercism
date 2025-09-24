pub struct PascalsTriangle {
	rows: u32,
}

impl PascalsTriangle {
	pub fn new(row_count: u32) -> Self {
		Self { rows: row_count }
	}

	pub fn rows(&self) -> Vec<Vec<u32>> {
		let mut sol: Vec<Vec<u32>> = vec![];

		if self.rows == 0 {
			return vec![];
		}

		for r in 0..self.rows {
			if r == 0 {
				sol.push(vec![1]);
			} else if r == 1 {
				sol.push(vec![1, 1]);
			} else {
				let row = (0..=r)
					.map(|idx| match idx {
						0 => 1,
						idx if idx == r => 1,
						idx => {
							sol[(r - 1) as usize][(idx - 1) as usize]
								+ sol[(r - 1) as usize][idx as usize]
						}
					})
					.collect::<Vec<_>>();
				sol.push(row);
			}
		}

		sol
	}
}
