#[derive(Debug)]
pub struct Item {
	pub weight: u32,
	pub value: u32,
}

// watch knapsack problem dynamic programming algorithm:
// https://www.youtube.com/watch?v=cJ21moQpofY
pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
	if items.is_empty() {
		return 0;
	}

	let mut knapsack_table: Vec<Vec<u32>> = vec![vec![0; max_weight as usize + 1]; items.len() + 1];

	for row in 0..=items.len() {
		for col in 0..=max_weight as usize {
			if row == 0 {
				knapsack_table[row][col] = 0;
				break;
			}
			let current_item = &items[row - 1];
			let prev_state_no_item = knapsack_table[row - 1][col];
			let prev_state_with_item = if col >= current_item.weight as usize {
				knapsack_table[row - 1][col - current_item.weight as usize] + current_item.value
			} else {
				0
			};
			knapsack_table[row][col] = prev_state_no_item.max(prev_state_with_item);
		}
	}

	knapsack_table[items.len()][max_weight as usize]
}
