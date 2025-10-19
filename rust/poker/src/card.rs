use std::{
	convert::TryFrom,
	cmp::{PartialOrd, Ord, Ordering, PartialEq, Eq},
	fmt::{self, Display},
};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub enum Suit {
	Spade,
	Heart,
	Club,
	Diamond,
}

impl Ord for Suit {
	fn cmp(&self, other: &Self) -> Ordering {
		let self_num = *self as u8;
		let other_num = *other as u8;

		match self_num.cmp(&other_num) {
			Ordering::Less => Ordering::Greater,
			Ordering::Equal => Ordering::Equal,
			Ordering::Greater => Ordering::Less,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub struct Card {
	suit: Suit,
	rank: u8,
}

pub enum CardConversionError {
	UnknownSuit(String),
	UnknownRank(String),
}

impl<'a> TryFrom<&'a str> for Card {
	type Error = CardConversionError;

	fn try_from(val: &'a str) -> Result<Self, Self::Error> {
		let upper_val = val.to_uppercase();

		let suit_str = upper_val.chars().last().ok_or(CardConversionError::UnknownSuit(upper_val.clone()))?;
		let suit = match suit_str {
			'S' => Suit::Spade,
			'H' => Suit::Heart,
			'C' => Suit::Club,
			'D' => Suit::Diamond,
			_ => {
				return Err(CardConversionError::UnknownSuit(upper_val));
			}
		};

		let rank_str = &upper_val[0..(upper_val.len()-1)];
		let rank: u8 = match rank_str {
			"A" => 1,
			"J" => 11,
			"Q" => 12,
			"K" => 13,
			"1" => {
				return Err(CardConversionError::UnknownRank(upper_val));
			},
			_ => rank_str.parse::<u8>().map_err(|_| CardConversionError::UnknownRank(upper_val.clone()))?
		};

		if rank < 1 || rank > 13 {
			return Err(CardConversionError::UnknownRank(upper_val));
		}

		Ok(Card { suit, rank })
	}
}

impl Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

		let rank_str = match self.rank {
			1 => "A",
			_ if self.rank >= 2 && self.rank <= 10 => &self.rank.to_string(),
			11 => "J",
			12 => "Q",
			13 => "K",
			_ => {
				return Err(fmt::Error);
			}
		};

		let suit_str = match self.suit {
			Suit::Spade => "S",
			Suit::Heart => "H",
			Suit::Club => "C",
			Suit::Diamond => "D",
		};

		write!(f, "{rank_str}{suit_str}")
	}
}

impl Ord for Card {
	fn cmp(&self, other: &Self) -> Ordering {
		match self.rank.cmp(&other.rank) {
			Ordering::Less => Ordering::Less,
			Ordering::Greater => Ordering::Greater,
			Ordering::Equal => self.suit.cmp(&other.suit),
		}
	}
}
