use core::cmp::Ordering;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand<'a> {
  hand_str: &'a str,
  cards: Vec<&'a str>
}

impl<'a> Hand<'a> {
  pub fn new(hand_str: &'a str) -> Result<Hand, String> {
    let mut cards = vec![];
    for card in hand_str.clone().split_whitespace() {
      if Hand::valid_card(card) {
        cards.push(card);
      } else {
        return Err(format!("{} is not a valid card", card));
      }
    }
    Ok(Hand { hand_str, cards })
  }

  pub fn ranking(&self, other: &Hand) -> Ordering {
    Ordering::Equal
  }

  fn valid_card(card: &str) -> bool {
    true
  }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
  let mut hands_in_struct: Vec<Hand> = hands
    .into_iter()
    .map(|hand_str| match Hand::new(hand_str) {
      Ok(hand) => hand,
      Err(msg) => panic!(msg),
    })
    .collect::<Vec<_>>();

  hands_in_struct.sort_unstable_by(|a, b| a.ranking(b));

  // Assume the first one is the largest, get the first one
  let largest = &hands_in_struct[0];
  let largest_set: Vec<&'a str> = hands_in_struct
    .iter()
    .filter(|hand| largest.ranking(&hand) == Ordering::Equal)
    .map(|hand| hand.hand_str)
    .collect::<Vec<_>>();

  Some(largest_set)
}
