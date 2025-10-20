use std::{
	cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
	convert::{TryFrom, TryInto},
};

use crate::{Card, card::CardConversionError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hand<'a> {
	TwoPairs(&'a str, [Card; 5]),
	OnePair(&'a str, [Card; 5]),
	HighCard(&'a str, [Card; 5]),
}

impl<'a> Ord for Hand<'a> {
	fn cmp(&self, other: &Self) -> Ordering {
		let self_rank = self.rank();
		let other_rank = other.rank();

		let cmp = self_rank.cmp(&other_rank);
		if cmp != Ordering::Equal {
			return cmp;
		}

		// The same rank
		match self {
			Hand::TwoPairs(_, cards) => {
				let self_pairs = find_pair_pos(cards);
				let other_pairs = find_pair_pos(other.cards());
				let other_cards = other.cards();

				let mut self_pair_rank =
					self_pairs.iter().map(|(i, _)| cards[*i].rank).collect::<Vec<_>>();
				self_pair_rank.sort();
				self_pair_rank = self_pair_rank.into_iter().rev().collect::<Vec<_>>();

				let mut other_pair_rank =
					other_pairs.iter().map(|(i, _)| other_cards[*i].rank).collect::<Vec<_>>();
				other_pair_rank.sort();
				other_pair_rank = other_pair_rank.into_iter().rev().collect::<Vec<_>>();

				for (self_rank, other_rank) in self_pair_rank.iter().zip(other_pair_rank.iter()) {
					let cmp = self_rank.cmp(other_rank);
					if cmp != Ordering::Equal {
						return cmp;
					}
				}

				cmp_card_by_card(
					&skip_cards(
						cards,
						&self_pairs.iter().fold(Vec::new(), |mut acc, (i, j)| {
							acc.append(vec![*i, *j].as_mut());
							acc
						}),
					),
					&skip_cards(
						other_cards,
						&other_pairs.iter().fold(Vec::new(), |mut acc, (i, j)| {
							acc.append(vec![*i, *j].as_mut());
							acc
						}),
					),
				)
			}
			Hand::OnePair(_, cards) => {
				let (self_i, self_j) = find_pair_pos(cards)[0];
				let (other_i, other_j) = find_pair_pos(other.cards())[0];

				let cmp = cards[self_i].rank.cmp(&other.cards()[other_i].rank);
				if cmp != Ordering::Equal {
					return cmp;
				}

				cmp_card_by_card(
					&skip_cards(cards, &[self_i, self_j]),
					&skip_cards(other.cards(), &[other_i, other_j]),
				)
			}
			Hand::HighCard(_, _) => cmp_card_by_card(self.cards(), other.cards()),
		}
	}
}

impl<'a> PartialOrd for Hand<'a> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum HandConversionError {
	IncorrectTokenNum,
	CardConversionError(CardConversionError),
}

impl<'a> TryFrom<&'a str> for Hand<'a> {
	type Error = HandConversionError;

	fn try_from(hand_str: &'a str) -> Result<Self, Self::Error> {
		if hand_str.split(" ").count() != 5 {
			return Err(HandConversionError::IncorrectTokenNum);
		}

		let mut cards = hand_str
			.split(" ")
			.map(Card::try_from)
			.collect::<Result<Vec<_>, _>>()
			.map_err(HandConversionError::CardConversionError)?;

		cards.sort();

		// converting to card array and sort card in descending order
		let cards: [Card; 5] = cards
			.into_iter()
			.rev()
			.take(5)
			.collect::<Vec<_>>()
			.try_into()
			.map_err(|_| HandConversionError::IncorrectTokenNum)?;

		Ok(match cards {
			cards if has_two_pairs(&cards) => Hand::TwoPairs(hand_str, cards),
			cards if has_one_pair(&cards) => Hand::OnePair(hand_str, cards),
			_ => Hand::HighCard(hand_str, cards),
		})
	}
}

impl<'a> Hand<'a> {
	pub fn to_src_str(&self) -> &'a str {
		match self {
			Hand::TwoPairs(src_str, _) => src_str,
			Hand::OnePair(src_str, _) => src_str,
			Hand::HighCard(src_str, _) => src_str,
		}
	}

	pub fn rank(&self) -> u8 {
		match self {
			Hand::TwoPairs(_, _) => 30,
			Hand::OnePair(_, _) => 20,
			Hand::HighCard(_, _) => 10,
		}
	}

	pub fn cards(&self) -> &[Card] {
		match self {
			Hand::TwoPairs(_, cards) => cards,
			Hand::OnePair(_, cards) => cards,
			Hand::HighCard(_, cards) => cards,
		}
	}
}

fn pair_num(cards: &[Card]) -> u8 {
	let mut buckets = vec![vec![]; 14];
	for (idx, card) in cards.iter().enumerate() {
		buckets[card.rank as usize].push(idx);
	}

	buckets.iter().filter(|bucket| bucket.len() == 2).count() as u8
}

fn has_two_pairs(cards: &[Card]) -> bool {
	pair_num(cards) == 2
}

fn has_one_pair(cards: &[Card]) -> bool {
	pair_num(cards) == 1
}

fn find_pair_pos(cards: &[Card]) -> Vec<(usize, usize)> {
	let mut buckets = vec![vec![]; 14];
	for (idx, card) in cards.iter().enumerate() {
		buckets[card.rank as usize].push(idx);
	}

	buckets
		.iter()
		.filter_map(|bucket| if bucket.len() == 2 { Some((bucket[0], bucket[1])) } else { None })
		.collect::<Vec<_>>()
}

fn skip_cards(cards: &[Card], skip: &[usize]) -> Vec<Card> {
	cards
		.iter()
		.enumerate()
		.filter_map(|(idx, card)| if skip.contains(&idx) { None } else { Some(card) })
		.cloned()
		.collect::<Vec<_>>()
}

fn cmp_card_by_card(first: &[Card], second: &[Card]) -> Ordering {
	let mut first = first.to_vec();
	first.sort();
	let first = first.iter().rev().collect::<Vec<_>>();

	let mut second = second.to_vec();
	second.sort();
	let second = second.iter().rev().collect::<Vec<_>>();

	for (first_card, second_card) in first.iter().zip(second.iter()) {
		let cmp = first_card.rank.cmp(&second_card.rank);
		if cmp != Ordering::Equal {
			return cmp;
		}
	}
	Ordering::Equal
}
