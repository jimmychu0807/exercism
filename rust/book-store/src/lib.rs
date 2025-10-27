use std::collections::HashMap;

const BK_PRICE: u32 = 800;
const DISCOUNTS_PCT: [u32; 6] = [100, 100, 95, 90, 80, 75];

pub fn lowest_price(books: &[u32]) -> u32 {
	let mut counts = [0u8; 5];
	for &b in books.iter() {
		if (1..=5).contains(&b) {
			counts[(b - 1) as usize] += 1;
		}
	}

	let mut memo: HashMap<[u8; 5], u32> = HashMap::new();
	rec_counts(counts, &mut memo)
}

fn rec_counts(counts: [u8; 5], memo: &mut HashMap<[u8; 5], u32>) -> u32 {
	// base
	if counts.iter().all(|&c| c == 0) {
		return 0;
	}
	if let Some(&v) = memo.get(&counts) {
		return v;
	}

	let mut best = u32::MAX;

	for mask in 1..(1 << 5) {
		let mut ok = true;
		let mut next = counts;
		let mut set_size = 0_usize;

		for (i, count) in next.iter_mut().enumerate() {
			if (mask >> i) & 1 == 1 {
				if *count == 0 {
					ok = false;
					break;
				}
				*count -= 1;
				set_size += 1;
			}
		}

		if !ok {
			continue;
		}

		let price = BK_PRICE * set_size as u32 * DISCOUNTS_PCT[set_size] / 100;
		let total = price + rec_counts(next, memo);
		if total < best {
			best = total;
		}
	}

	memo.insert(counts, best);
	best
}
