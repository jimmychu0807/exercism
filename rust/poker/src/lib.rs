use std::cmp::Ordering;

mod card;
pub use card::Card;

mod hand;
pub use hand::Hand;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
	let mut hands = hands
		.iter()
		.map(|&hand_str| Hand::try_from(hand_str))
		.collect::<Result<Vec<Hand>, _>>()
		.unwrap();

	hands.sort();
	hands = hands.into_iter().rev().collect::<Vec<_>>();

	if hands.is_empty() {
		return vec![];
	}

	// pick the largest one and put in the vector
	let mut res = vec![hands.remove(0)];
	for hand in hands.into_iter() {
		if res[0].cmp(&hand) == Ordering::Equal {
			res.push(hand);
		}
	}

	res.into_iter().map(|hand| hand.to_src_str()).collect()
}
