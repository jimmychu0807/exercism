use std::cmp::{max, min};

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
	let mut sol: Vec<(usize, usize)> = vec![];

	let ttl_rows = input.len();
	let ttl_cols = if ttl_rows == 0 { 0 } else { input[0].len() };

	for (rdx, row) in input.iter().enumerate() {
		for cdx in 0..ttl_cols {
			let col = get_col(input, cdx);
			let val = row[cdx];
			if val == get_max(row) && val == get_min(&col) {
				sol.push((rdx, cdx));
			}
		}
	}

	sol
}

fn get_col(input: &[Vec<u64>], cdx: usize) -> Vec<u64> {
	input.iter().map(|row| row[cdx]).collect::<Vec<_>>()
}

fn get_max(row: &[u64]) -> u64 {
	if row.is_empty() {
		panic!("get_max() called on an empty row");
	}
	row.iter().fold(row[0], |acc, v| max(acc, *v))
}

fn get_min(row: &[u64]) -> u64 {
	if row.is_empty() {
		panic!("get_min() called on an empty row");
	}
	row.iter().fold(row[0], |acc, v| min(acc, *v))
}
