use core::cmp::{ PartialOrd, PartialEq, Ordering };
use std::fmt;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

// J - 11
// Q - 12
// K - 13
// A - 1, 14

#[derive(Debug)]
enum PokerHandPattern {
  StraightFlush(u32),
  FourOfAKind(u32, u32),
  FullHouse(u32, u32),
  Flush(u32, u32, u32, u32, u32),
  Straight(u32),
  ThreeOfAKind(u32, u32, u32),
  TwoPair(u32, u32, u32),
  OnePair(u32, u32, u32, u32),
  Nothing(u32, u32, u32, u32, u32),
}

impl PartialOrd for PokerHandPattern {
  fn partial_cmp(&self, other: &PokerHandPattern) -> Option<Ordering> {
    Some(Ordering::Less)
  }
}

impl PartialEq for PokerHandPattern {
  fn eq(&self, other: &PokerHandPattern) -> bool {
    true
  }
}

#[derive(Eq, Debug)]
struct Card {
  rank: u32,
  suit: String,
}

impl Card {
  fn new<'a>(card_str: &'a str) -> Card {
    let card_str = card_str.to_uppercase();
    let suit = card_str.chars().last().unwrap().to_string();
    let rank_str = card_str.chars().take(card_str.chars().count() - 1).collect::<String>();
    let rank = rank_str.parse::<u32>().or_else(|_| {
      match rank_str {
        rank_str if rank_str == "J" => Ok(11),
        rank_str if rank_str == "Q" => Ok(12),
        rank_str if rank_str == "K" => Ok(13),
        rank_str if rank_str == "A" => Ok(14),
        _ => Err(())
      }
    }).unwrap();
    Card { rank, suit }
  }
}

impl Ord for Card {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.rank != other.rank { return self.rank.cmp(&other.rank) }

    // closure
    let suit_rank = |suit_str| match suit_str {
      s if s == "S" => 4,
      s if s == "H" => 3,
      s if s == "C" => 2,
      _ => 1
    };
    let self_suit_rank = suit_rank(&self.suit);
    let other_suit_rank = suit_rank(&other.suit);
    self_suit_rank.cmp(&other_suit_rank)
  }
}

impl PartialOrd for Card {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Card {
  fn eq(&self, other: &Self) -> bool {
    self.rank == other.rank && self.suit == other.suit
  }
}

#[derive(Debug)]
struct PokerHand<'a> {
  hand_str: &'a str,
  cards: Vec<Card>,
  pattern: PokerHandPattern,
}

impl<'a> PokerHand<'a> {
  fn new(hand_str: &'a str) -> PokerHand {
    let cards = Self::get_sorted_cards(hand_str);
    PokerHand { hand_str, cards, pattern: Self::get_pattern(hand_str) }
  }

  fn get_sorted_cards(hand_str: &'a str) -> Vec<Card> {
    let mut cards = hand_str.split(' ')
      .map(|card_str| Card::new(card_str))
      .collect::<Vec<_>>();
    cards.sort();
    cards
  }

  fn get_pattern(hand: &'a str) -> PokerHandPattern {
    // TODO: fill in actual implementation
    PokerHandPattern::StraightFlush(14)
  }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
  if hands.len() == 0 { return None }

  let mut hands = hands.iter().map(|hand| PokerHand::new(*hand)).collect::<Vec<_>>();
  hands.sort_by(|a, b| b.pattern.partial_cmp(&a.pattern).unwrap());
  let largest = hands.first().unwrap();

  println!("hands: {:?}", hands);
  println!("largest: {:?}", largest);

  // could be multiple answers, so we want to retrieve them all
  let res = hands
    .iter()
    .filter(|hand| hand.pattern.eq(&largest.pattern))
    .map(|hand| hand.hand_str)
    .collect::<Vec<_>>();

  Some(res)
}
