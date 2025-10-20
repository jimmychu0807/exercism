use std::{
	cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
	convert::{TryFrom, TryInto},
};

use crate::{Card, card::CardConversionError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hand<'a> {
	RoyalFlush(&'a str, [Card; 5]),
	StraightFlush(&'a str, [Card; 5]),
	FourOfAKind(&'a str, [Card; 5]),
	FullHouse(&'a str, [Card; 5]),
	Flush(&'a str, [Card; 5]),
	Straight(&'a str, [Card; 5]),
	ThreeOfAKind(&'a str, [Card; 5]),
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
			Hand::RoyalFlush(_, _) => Ordering::Equal,
			Hand::StraightFlush(_, cards) => {
				let self_rank = highest_rank_of_straight(cards).unwrap();
				let other_rank = highest_rank_of_straight(other.cards()).unwrap();
				self_rank.cmp(&other_rank)
			}
			Hand::FourOfAKind(_, cards) => {
				let self_quadruplet = &find_same_rank_pos(cards, 4)[0];

				let other_cards = other.cards();
				let other_quadruplet = &find_same_rank_pos(other_cards, 4)[0];

				let cmp =
					cards[self_quadruplet[0]].rank.cmp(&other_cards[other_quadruplet[0]].rank);
				if cmp != Ordering::Equal {
					return cmp;
				}

				cmp_card_by_card(
					&skip_cards(cards, self_quadruplet),
					&skip_cards(other_cards, other_quadruplet),
				)
			}
			Hand::FullHouse(_, cards) => {
				let other_cards = other.cards();

				let self_triplet = &find_same_rank_pos(cards, 3)[0];
				let other_triplet = &find_same_rank_pos(other_cards, 3)[0];
				let cmp = cards[self_triplet[0]].rank.cmp(&other_cards[other_triplet[0]].rank);
				if cmp != Ordering::Equal {
					return cmp;
				}

				let self_pair = &find_same_rank_pos(cards, 2)[0];
				let other_pair = &find_same_rank_pos(other_cards, 2)[0];
				cards[self_pair[0]].rank.cmp(&other_cards[other_pair[0]].rank)
			}
			Hand::Flush(_, cards) => cmp_card_by_card(cards, other.cards()),
			Hand::Straight(_, cards) => {
				let self_rank = highest_rank_of_straight(cards).unwrap();
				let other_rank = highest_rank_of_straight(other.cards()).unwrap();
				self_rank.cmp(&other_rank)
			}
			Hand::ThreeOfAKind(_, cards) => {
				let self_triplet = &find_same_rank_pos(cards, 3)[0];

				let other_cards = other.cards();
				let other_triplet = &find_same_rank_pos(other_cards, 3)[0];

				let cmp = cards[self_triplet[0]].rank.cmp(&other_cards[other_triplet[0]].rank);
				if cmp != Ordering::Equal {
					return cmp;
				}

				cmp_card_by_card(
					&skip_cards(cards, self_triplet),
					&skip_cards(other_cards, other_triplet),
				)
			}
			Hand::TwoPairs(_, cards) => {
				let self_pairs = find_same_rank_pos(cards, 2);
				let other_pairs = find_same_rank_pos(other.cards(), 2);
				let other_cards = other.cards();

				let mut self_pair_rank =
					self_pairs.iter().map(|bucket| cards[bucket[0]].rank).collect::<Vec<_>>();
				self_pair_rank.sort();
				self_pair_rank = self_pair_rank.into_iter().rev().collect::<Vec<_>>();

				let mut other_pair_rank = other_pairs
					.iter()
					.map(|bucket| other_cards[bucket[0]].rank)
					.collect::<Vec<_>>();
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
						&self_pairs.iter().fold(Vec::new(), |acc, bucket| {
							bucket.iter().fold(acc, |mut acc, pos| {
								acc.push(*pos);
								acc
							})
						}),
					),
					&skip_cards(
						other_cards,
						&other_pairs.iter().fold(Vec::new(), |acc, bucket| {
							bucket.iter().fold(acc, |mut acc, pos| {
								acc.push(*pos);
								acc
							})
						}),
					),
				)
			}
			Hand::OnePair(_, cards) => {
				let self_pair = &find_same_rank_pos(cards, 2)[0];
				let other_pair = &find_same_rank_pos(other.cards(), 2)[0];

				let cmp = cards[self_pair[0]].rank.cmp(&other.cards()[other_pair[0]].rank);
				if cmp != Ordering::Equal {
					return cmp;
				}

				cmp_card_by_card(
					&skip_cards(cards, self_pair),
					&skip_cards(other.cards(), other_pair),
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
			cards
				if is_straight(&cards)
					&& has_same_suit(&cards)
					&& cards[0].rank == 14
					&& cards[1].rank == 13 =>
			{
				Hand::RoyalFlush(hand_str, cards)
			}
			cards if is_straight(&cards) && has_same_suit(&cards) => {
				Hand::StraightFlush(hand_str, cards)
			}
			cards if has_four_of_a_kind(&cards) => Hand::FourOfAKind(hand_str, cards),
			cards if has_three_of_a_kind(&cards) && has_one_pair(&cards) => {
				Hand::FullHouse(hand_str, cards)
			}
			cards if has_same_suit(&cards) => Hand::Flush(hand_str, cards),
			cards if is_straight(&cards) => Hand::Straight(hand_str, cards),
			cards if has_three_of_a_kind(&cards) => Hand::ThreeOfAKind(hand_str, cards),
			cards if has_two_pairs(&cards) => Hand::TwoPairs(hand_str, cards),
			cards if has_one_pair(&cards) => Hand::OnePair(hand_str, cards),
			_ => Hand::HighCard(hand_str, cards),
		})
	}
}

impl<'a> Hand<'a> {
	pub fn to_src_str(&self) -> &'a str {
		match self {
			Hand::RoyalFlush(src_str, _) => src_str,
			Hand::StraightFlush(src_str, _) => src_str,
			Hand::FourOfAKind(src_str, _) => src_str,
			Hand::FullHouse(src_str, _) => src_str,
			Hand::Flush(src_str, _) => src_str,
			Hand::Straight(src_str, _) => src_str,
			Hand::ThreeOfAKind(src_str, _) => src_str,
			Hand::TwoPairs(src_str, _) => src_str,
			Hand::OnePair(src_str, _) => src_str,
			Hand::HighCard(src_str, _) => src_str,
		}
	}

	pub fn rank(&self) -> u8 {
		match self {
			Hand::RoyalFlush(_, _) => 90,
			Hand::StraightFlush(_, _) => 80,
			Hand::FourOfAKind(_, _) => 70,
			Hand::FullHouse(_, _) => 60,
			Hand::Flush(_, _) => 50,
			Hand::Straight(_, _) => 45,
			Hand::ThreeOfAKind(_, _) => 40,
			Hand::TwoPairs(_, _) => 30,
			Hand::OnePair(_, _) => 20,
			Hand::HighCard(_, _) => 10,
		}
	}

	pub fn cards(&self) -> &[Card] {
		match self {
			Hand::RoyalFlush(_, cards) => cards,
			Hand::StraightFlush(_, cards) => cards,
			Hand::FourOfAKind(_, cards) => cards,
			Hand::FullHouse(_, cards) => cards,
			Hand::Flush(_, cards) => cards,
			Hand::Straight(_, cards) => cards,
			Hand::ThreeOfAKind(_, cards) => cards,
			Hand::TwoPairs(_, cards) => cards,
			Hand::OnePair(_, cards) => cards,
			Hand::HighCard(_, cards) => cards,
		}
	}
}

fn is_straight(cards: &[Card]) -> bool {
	// Card should be sorted in descending when passing in
	let ranks = cards.iter().map(|c| c.rank).collect::<Vec<_>>();

	// handling A, 2, 3, 4, 5
	if ranks == [14, 5, 4, 3, 2].to_vec() {
		return true;
	}

	for idx in 0..(ranks.len() - 1) {
		if ranks[idx] != ranks[idx + 1] + 1 {
			return false;
		}
	}
	true
}

fn highest_rank_of_straight(cards: &[Card]) -> Option<u8> {
	if !is_straight(cards) {
		return None;
	}

	// assume cards are sorted in descending order
	if cards[0].rank != 14 {
		return Some(cards[0].rank);
	}

	// cards[0] is `A`
	match cards[1].rank {
		13 => Some(14),
		5 => Some(5),
		_ => panic!("unexpected code path"),
	}
}

fn has_same_suit(cards: &[Card]) -> bool {
	cards.iter().all(|c| cards[0].suit == c.suit)
}

fn has_four_of_a_kind(cards: &[Card]) -> bool {
	find_same_rank_pos(cards, 4).len() == 1
}

fn has_three_of_a_kind(cards: &[Card]) -> bool {
	find_same_rank_pos(cards, 3).len() == 1
}

fn has_two_pairs(cards: &[Card]) -> bool {
	find_same_rank_pos(cards, 2).len() == 2
}

fn has_one_pair(cards: &[Card]) -> bool {
	find_same_rank_pos(cards, 2).len() == 1
}

fn find_same_rank_pos(cards: &[Card], rank_num: usize) -> Vec<Vec<usize>> {
	let mut buckets = vec![vec![]; 15];
	for (idx, card) in cards.iter().enumerate() {
		buckets[card.rank as usize].push(idx);
	}

	buckets
		.into_iter()
		.filter_map(|bucket| if bucket.len() == rank_num { Some(bucket) } else { None })
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
