use core::cmp::{ PartialOrd, PartialEq, Ordering };
use std::fmt;
use std::collections:{ HashMap };

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
  // StraightFlush(u32),
  FourOfAKind(u32, u32),
  FullHouse(u32, u32),
  // Flush(u32, u32, u32, u32, u32),
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

#[derive(Eq, Debug, Clone)]
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
  pub fn new(hand_str: &'a str) -> PokerHand {
    let cards = Self::get_sorted_cards(hand_str);
    let mut hand = PokerHand {
      hand_str,
      cards: cards.clone(),
      pattern: Self::get_pattern(&cards)
    };

  }

  fn get_sorted_cards(hand_str: &'a str) -> Vec<Card> {
    let mut cards = hand_str.split(' ')
      .map(|card_str| Card::new(card_str))
      .collect::<Vec<_>>();
    cards.sort();
    cards.reverse();
    cards
  }

  pub fn get_suit_hashmap(&self) -> HashMap {
    Self::_get_suit_hashmap(&self.cards)
  }

  fn _get_suit_hashmap(cards: &Vec<Card>) -> HashMap {
    let mut map = HashMap::new();
    cards.iter().for_each(|card| match map.get_mut(card.suit) {
      Some(val) => *val += 1,
      None => map.insert(card.suit, 1),
    });
    map
  }

  pub fn get_rank_hashmap(&self) -> HashMap {
    Self::_get_rank_hashmap(&self.cards)
  }

  fn _get_rank_hashmap(cards: &Vec<Card>) -> HashMap {
    let mut map = HashMap::new();
    cards.iter().for_each(|card| match map.get_mut(card.rank) {
      Some(val) => *val += 1,
      None => map.insert(card.rank, 1),
    });
    map
  }

  fn get_pattern(cards: &Vec<Card>) -> PokerHandPattern {
    // TODO: fill in actual implementation
    let suit_hashmap = Self::_get_suit_hashmap(cards);
    let rank_hashmap = Self::_get_rank_hashmap(cards);

    let consecutive_highest = |cards| {
      // assume cards are sorted, highest to lowest
      let highest = cards[0].rank;
      if (cards[1].rank == highest - 1 &&
        cards[2].rank == highest - 2 &&
        cards[3].rank == highest - 3 &&
        cards[4].rank == highest - 4) { return Some(highest) }

      // check A, 5, 4, 3, 2
      if (cards[0].rank == 14 &&
        cards[1].rank == 5 &&
        cards[2].rank == 4 &&
        cards[3].rank == 3 &&
        cards[4].rank == 2) { return Some(5) }

      return None
    }

    // distinguish from highest to lowest
    if suit_hashmap.keys().count() == 1 {
      if rank_hashmap.keys().count() == 5 {
        match consecutive_highest(cards) {
          Some(val) => return PokerHandPattern::StraightFlush(val),
          None => return PokerHandPattern::Flush(cards.map(|c| c.rank)),
        }
      }
    } else if rank_hashmap.keys().count() == 2 {
      // distinguish FourOfAKind or FullHouse
    }

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
