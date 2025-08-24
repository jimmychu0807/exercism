pub fn annotate(garden: &[&str]) -> Vec<String> {
	if garden.is_empty() {
		return vec![];
	}

	let mut result: Vec<String> = vec![];

	let num_row = garden.len();
	let num_col = garden[0].len();

	for row_idx in 0..num_row {
		let mut result_row = String::from("");

		for col_idx in 0..num_col {
			// we are looping thru each cell
			let mut num_flower = 0_u32;

			// if it is a `*` just push that and check the next square
			if garden[row_idx].as_bytes()[col_idx] == 42 {
				result_row.push('*');
				continue;
			}

			// looping over the neighboring eight cells
			for di in [-1_i32, 0, 1] {
				for dj in [-1_i32, 0, 1] {
					if di == 0 && dj == 0 {
						continue;
					}

					let neighbor_row = dj + row_idx as i32;
					let neighbor_col = di + col_idx as i32;

					if neighbor_row < 0 || neighbor_row >= num_row as i32 {
						continue;
					}
					if neighbor_col < 0 || neighbor_col >= num_col as i32 {
						continue;
					}
					if garden[neighbor_row as usize].as_bytes()[neighbor_col as usize] == 42 {
						num_flower += 1;
					}
				}
			}

			let target_char =
				if num_flower == 0 { ' ' } else { char::from_digit(num_flower, 10).unwrap() };
			result_row.push(target_char);
		}
		result.push(result_row);
	}

	result
}
