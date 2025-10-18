use std::convert::{TryFrom, TryInto};

pub enum Hand {
	TwoPair([String; 5]),
	OnePair([String; 5]),
	HighCard([String; 5]),
}

pub struct HandConversionError;

impl TryFrom<&str> for Hand {
	type Error = HandConversionError;

	fn try_from(hand_str: &str) -> Result<Self, Self::Error> {

		Ok(Hand::HighCard(
			["1", "2", "3", "4", "5"].iter().map(|c| c.to_string()).collect::<Vec<_>>().try_into().unwrap()
		))
	}
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
	let hands = hands.into_iter().filter_map(|hand_str|
		TryInto::<Hand>::try_into(*hand_str).ok()
	).collect::<Vec<_>>();

	Some(Vec::new())
}
