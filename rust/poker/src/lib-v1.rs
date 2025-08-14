use core::cmp::{ PartialOrd, PartialEq, Ordering };
use std::collections::HashMap;

#[derive(Debug)]
pub enum PokerHandPattern {
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
    match self {
      PokerHandPattern::StraightFlush(s1) => match other {
        PokerHandPattern::StraightFlush(o1) => s1.partial_cmp(o1),
        _ => Some(Ordering::Greater),
      },
      PokerHandPattern::FourOfAKind(s1, s2) => match other {
        PokerHandPattern::StraightFlush(..) => Some(Ordering::Less),
        PokerHandPattern::FourOfAKind(o1, o2) =>
          [s1, s2].to_vec().partial_cmp(&[o1, o2].to_vec()),
        _ => Some(Ordering::Greater),
      },
      PokerHandPattern::FullHouse(s1, s2) => match other {
        PokerHandPattern::StraightFlush(..) | PokerHandPattern::FourOfAKind(..) =>
          Some(Ordering::Less),
        PokerHandPattern::FullHouse(o1, o2) =>
          [s1, s2].to_vec().partial_cmp(&[o1, o2].to_vec()),
        _ => Some(Ordering::Greater),
      },
      PokerHandPattern::Flush(s1, s2, s3, s4, s5) => match other {
        PokerHandPattern::StraightFlush(..) | PokerHandPattern::FourOfAKind(..) |
          PokerHandPattern::FullHouse(..) => Some(Ordering::Less),
        PokerHandPattern::Flush(o1, o2, o3, o4, o5) =>
          [s1, s2, s3, s4, s5].to_vec().partial_cmp(&[o1, o2, o3, o4, o5].to_vec()),
        _ => Some(Ordering::Greater),
      },
      PokerHandPattern::Straight(s1) => match other {
        PokerHandPattern::StraightFlush(..) | PokerHandPattern::FourOfAKind(..) |
          PokerHandPattern::FullHouse(..) | PokerHandPattern::Flush(..) => Some(Ordering::Less),
        PokerHandPattern::Straight(o1) => s1.partial_cmp(o1),
        _ => Some(Ordering::Greater),
      },
      PokerHandPattern::ThreeOfAKind(s1, s2, s3) => match other {
        PokerHandPattern::StraightFlush(..) | PokerHandPattern::FourOfAKind(..) |
          PokerHandPattern::FullHouse(..) | PokerHandPattern::Flush(..) |
          PokerHandPattern::Straight(..) => Some(Ordering::Less),
        PokerHandPattern::ThreeOfAKind(o1, o2, o3) =>
          [s1, s2, s3].to_vec().partial_cmp(&[o1, o2, o3].to_vec()),
        _ => Some(Ordering::Greater),
      },
      PokerHandPattern::TwoPair(s1, s2, s3) => match other {
        PokerHandPattern::OnePair(..) | PokerHandPattern::Nothing(..) =>
          Some(Ordering::Greater),
        PokerHandPattern::TwoPair(o1, o2, o3) =>
          [s1, s2, s3].to_vec().partial_cmp(&[o1, o2, o3].to_vec()),
        _ => Some(Ordering::Less),
      },
      PokerHandPattern::OnePair(s1, s2, s3, s4) => match other {
        PokerHandPattern::Nothing(..) => Some(Ordering::Greater),
        PokerHandPattern::OnePair(o1, o2, o3, o4) =>
          [s1, s2, s3, s4].to_vec().partial_cmp(&[o1, o2, o3, o4].to_vec()),
        _ => Some(Ordering::Less),
      },
      PokerHandPattern::Nothing(s1, s2, s3, s4, s5) => match other {
        PokerHandPattern::Nothing(o1, o2, o3, o4, o5) =>
          [s1, s2, s3, s4, s5].to_vec().partial_cmp(&[o1, o2, o3, o4, o5].to_vec()),
        _ => Some(Ordering::Less),
      },
    }
  }
}

impl PartialEq for PokerHandPattern {
  fn eq(&self, other: &PokerHandPattern) -> bool {
    match self {
      PokerHandPattern::StraightFlush(sval) => match other {
        PokerHandPattern::StraightFlush(oval) => sval == oval,
        _ => false,
      },
      PokerHandPattern::FourOfAKind(sval1, sval2) => match other {
        PokerHandPattern::FourOfAKind(oval1, oval2) => sval1 == oval1 && sval2 == oval2,
        _ => false,
      },
      PokerHandPattern::FullHouse(sval1, sval2) => match other {
        PokerHandPattern::FullHouse(oval1, oval2) => sval1 == oval1 && sval2 == oval2,
        _ => false,
      },
      PokerHandPattern::Flush(sval1, sval2, sval3, sval4, sval5) => match other {
        PokerHandPattern::Flush(oval1, oval2, oval3, oval4, oval5) =>
          sval1 == oval1 && sval2 == oval2 && sval3 == oval3 && sval4 == oval4 && sval5 == oval5,
        _ => false,
      },
      PokerHandPattern::Straight(sval1) => match other {
        PokerHandPattern::Straight(oval1) => sval1 == oval1,
        _ => false,
      },
      PokerHandPattern::ThreeOfAKind(sval1, sval2, sval3) => match other {
        PokerHandPattern::ThreeOfAKind(oval1, oval2, oval3) =>
          sval1 == oval1 && sval2 == oval2 && sval3 == oval3,
        _ => false,
      },
      PokerHandPattern::TwoPair(sval1, sval2, sval3) => match other {
        PokerHandPattern::TwoPair(oval1, oval2, oval3) =>
          sval1 == oval1 && sval2 == oval2 && sval3 == oval3,
        _ => false,
      },
      PokerHandPattern::OnePair(sval1, sval2, sval3, sval4) => match other {
        PokerHandPattern::OnePair(oval1, oval2, oval3, oval4) =>
          sval1 == oval1 && sval2 == oval2 && sval3 == oval3 && sval4 == oval4,
        _ => false,
      },
      PokerHandPattern::Nothing(sval1, sval2, sval3, sval4, sval5) => match other {
        PokerHandPattern::Nothing(oval1, oval2, oval3, oval4, oval5) =>
          sval1 == oval1 && sval2 == oval2 && sval3 == oval3 && sval4 == oval4 && sval5 == oval5,
        _ => false,
      },
    }
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
    // check the card is valid
    assert!(["S", "H", "C", "D"].contains(&suit.as_str()), format!("Unknown suit: {}", suit));

    let rank_str = card_str.chars().take(card_str.chars().count() - 1).collect::<String>();
    let rank = rank_str.parse::<u32>().or_else(|_| {
      match rank_str {
        rank_str if rank_str == "J" => Ok(11),
        rank_str if rank_str == "Q" => Ok(12),
        rank_str if rank_str == "K" => Ok(13),
        rank_str if rank_str == "A" => Ok(14),
        rank_str => Err(format!("Unknown rank_str: {}", rank_str)),
      }
    }).expect("Unknown rank_str");

    assert!(rank >= 2 && rank <= 14, format!("Invalid rank: {}", rank));

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
      s if s == "D" => 1,
      s => panic!("Unknown suit: {}", s),
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

pub type SuitHashMap = HashMap<String, u32>;
pub type RankHashMap = HashMap<u32, u32>;

#[derive(Debug)]
pub struct PokerHand<'a> {
  hand_str: &'a str,
  cards: Vec<Card>,
  pattern: PokerHandPattern,
}

impl<'a> PartialOrd for PokerHand<'a> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.pattern.partial_cmp(&other.pattern)
  }
}

impl<'a> PartialEq for PokerHand<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.pattern.eq(&other.pattern)
  }
}

impl<'a> PokerHand<'a> {
  pub fn new(hand_str: &'a str) -> PokerHand {
    let cards = Self::get_sorted_cards(hand_str);
    PokerHand {
      hand_str,
      cards: cards.clone(),
      pattern: Self::get_pattern(&cards)
    }
  }

  // The sort goes from highest to lowest
  fn get_sorted_cards(hand_str: &'a str) -> Vec<Card> {
    let mut cards = hand_str.split(' ')
      .map(|card_str| Card::new(card_str))
      .collect::<Vec<_>>();
    cards.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    cards
  }

  #[allow(dead_code)]
  pub fn get_suit_hashmap(&self) -> SuitHashMap {
    Self::_get_suit_hashmap(&self.cards)
  }

  fn _get_suit_hashmap(cards: &Vec<Card>) -> SuitHashMap {
    let mut map = HashMap::new();
    cards.iter().for_each(|card| match map.get_mut(&card.suit) {
      Some(val) => { *val += 1; },
      None => { map.insert(card.suit.clone(), 1); },
    });
    map
  }

  #[allow(dead_code)]
  pub fn get_rank_hashmap(&self) -> RankHashMap {
    Self::_get_rank_hashmap(&self.cards)
  }

  fn _get_rank_hashmap(cards: &Vec<Card>) -> RankHashMap {
    let mut map = HashMap::new();
    cards.iter().for_each(|card| match map.get_mut(&card.rank) {
      Some(val) => { *val += 1; },
      None => { map.insert(card.rank, 1); },
    });
    map
  }

  // Assumption, `cards` are already sorted
  fn get_pattern(cards: &Vec<Card>) -> PokerHandPattern {
    // TODO: fill in actual implementation
    let suit_hashmap = Self::_get_suit_hashmap(cards);
    let rank_hashmap = Self::_get_rank_hashmap(cards);

    let consecutive_highest = |cards: &Vec<Card>| {
      // assume cards are sorted, highest to lowest
      let highest = cards[0].rank;
      if cards[1].rank == highest - 1 &&
        cards[2].rank == highest - 2 &&
        cards[3].rank == highest - 3 &&
        cards[4].rank == highest - 4 { return Some(highest) }

      // check A, 5, 4, 3, 2
      if cards[0].rank == 14 &&
        cards[1].rank == 5 &&
        cards[2].rank == 4 &&
        cards[3].rank == 3 &&
        cards[4].rank == 2 { return Some(5) }

      return None
    };

    // distinguish from highest to lowest
    if suit_hashmap.keys().count() == 1 {
      return match consecutive_highest(cards) {
        Some(val) => PokerHandPattern::StraightFlush(val),
        None => PokerHandPattern::Flush(
          cards[0].rank, cards[1].rank, cards[2].rank, cards[3].rank, cards[4].rank),
      };
    } else if let Some(val) = consecutive_highest(cards) {
      return PokerHandPattern::Straight(val);

    } else if rank_hashmap.keys().count() == 2 {
      // distinguish FourOfAKind or FullHouse
      let first_key = rank_hashmap.keys().nth(0).unwrap();
      let first_val = rank_hashmap.get(first_key).unwrap();
      let last_key = rank_hashmap.keys().last().unwrap();
      return match first_val {
        1 => PokerHandPattern::FourOfAKind(*last_key, *first_key),
        2 => PokerHandPattern::FullHouse(*last_key, *first_key),
        3 => PokerHandPattern::FullHouse(*first_key, *last_key),
        4 => PokerHandPattern::FourOfAKind(*first_key, *last_key),
        _ => panic!("Unexpected result"),
      };
    } else if rank_hashmap.keys().count() == 3 {
      // either Three of a kind or Two pair
      if rank_hashmap.values().any(|f| *f == 3) {
        // Three of a kind
        let triple = rank_hashmap.iter().filter(|(_, f)| **f == 3).nth(0).unwrap();

        let mut rest_cards = cards.iter().filter(|c| c.rank != *triple.0).collect::<Vec<_>>();
        rest_cards.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        let rest_vals = rest_cards.iter().map(|c| c.rank).collect::<Vec<_>>();
        return PokerHandPattern::ThreeOfAKind(*triple.0, rest_vals[0], rest_vals[1]);
      }
      // Two Pair
      let doubles = rank_hashmap.iter().filter(|(_, f)| **f == 2).collect::<Vec<_>>();
      let mut double_vals: Vec<u32> = doubles.iter().map(|(r, _)| **r).collect::<Vec<_>>();
      double_vals.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

      let rest_card = cards.iter().filter(|c| !double_vals.contains(&c.rank)).nth(0).unwrap();
      return PokerHandPattern::TwoPair(double_vals[0], double_vals[1], rest_card.rank);

    } else if rank_hashmap.keys().count() == 4 {
      // One pair
      let pair = rank_hashmap.iter().filter(|(_, f)| **f == 2).nth(0).unwrap();

      let mut rest_cards = cards.iter().filter(|c| c.rank != *pair.0).collect::<Vec<_>>();
      rest_cards.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
      let rest_vals = rest_cards.iter().map(|c| c.rank).collect::<Vec<_>>();
      return PokerHandPattern::OnePair(*pair.0, rest_vals[0], rest_vals[1], rest_vals[2]);
    }
    // Nothing
    return PokerHandPattern::Nothing(
      cards[0].rank, cards[1].rank, cards[2].rank, cards[3].rank, cards[4].rank);
  }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
  if hands.len() == 0 { return None }

  let mut hands = hands.iter().map(|hand| PokerHand::new(*hand)).collect::<Vec<_>>();
  hands.sort_by(|a, b| b.partial_cmp(&a).unwrap());
  let largest = hands.first().unwrap();

  // could be multiple answers, so we want to retrieve them all
  let res = hands
    .iter()
    .filter(|hand| hand.pattern.eq(&largest.pattern))
    .map(|hand| hand.hand_str)
    .collect::<Vec<_>>();

  Some(res)
}
