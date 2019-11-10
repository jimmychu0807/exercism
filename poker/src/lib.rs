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

enum PokerHandPattern {
  StraightFlush(u32),
  FourOfAKind(u32,u32),
  FullHouse(u32, u32),
  Flush(Vec<u32>),
  PatternNone(Vec<u32>),
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

struct PokerHand<'a> {
  hand_str: &'a str,
  pattern: PokerHandPattern,
}

impl<'a> PokerHand<'a> {
  fn new(hand: &'a str) -> PokerHand {
    PokerHand {
      hand_str: hand,
      pattern: PokerHandPattern::PatternNone(vec![]),
    }
  }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
  if hands.len() == 0 { return None }

  let mut hands = hands.iter().map(|hand| PokerHand::new(*hand)).collect::<Vec<_>>();
  hands.sort_by(|a, b| b.pattern.partial_cmp(&a.pattern).unwrap());
  let largest = hands.first().unwrap();

  // could be multiple answer, we want to retrieve them all
  let res = hands
    .iter()
    .filter(|hand| hand.pattern.eq(&largest.pattern))
    .map(|hand| hand.hand_str)
    .collect::<Vec<_>>();

  Some(res)
}
