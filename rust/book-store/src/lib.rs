use std::collections::HashSet;

const BK_PRICE: u32 = 800;
const DISCOUNTS: &[f32] = &[1.0, 1.0, 0.95, 0.9, 0.8, 0.75];

pub fn lowest_price(books: &[u32]) -> u32 {
	let mut book_sets: Vec<HashSet<u32>> = Vec::new();

	'outer: for book in books.iter() {
		for book_set in book_sets.iter_mut() {
			if !book_set.contains(book) {
				book_set.insert(*book);
				continue 'outer;
			}
		}

		// doesn't break from the above, so we create a new hashset
		let mut set = HashSet::new();
		set.insert(*book);
		book_sets.push(set);
	}

	calc_book_sets_price(&book_sets)
}

fn calc_book_sets_price(book_sets: &[HashSet<u32>]) -> u32 {
	let discount = |val: u32, len: usize| -> u32 { (val as f32 * DISCOUNTS[len]) as u32 };

	book_sets.iter().fold(0, |acc, set| acc + discount(BK_PRICE * set.len() as u32, set.len()))
}
