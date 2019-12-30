use core::cmp::Ordering;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand<'a> {
  hand_str: &'a str,
  cards: Vec<&'a str>
}

const ERROR_MSGS: [&str; 3] = [
  "is not a valid card.",
  "does not contain a valid rank.",
  "does not contain a valid suit.",
];

impl<'a> Hand<'a> {
  pub fn new(hand_str: &'a str) -> Result<Hand, String> {
    let mut cards = vec![];
    for card in hand_str.clone().split_whitespace() {
      if Hand::valid_card(card) {
        cards.push(card);
      } else {
        return Err(format!("{} {}", card, ERROR_MSGS[0]));
      }
    }
    Ok(Hand { hand_str, cards })
  }

  pub fn ranking(&self, other: &Hand) -> Ordering {
    Ordering::Equal
  }

  fn valid_card(card: &str) -> bool {
    if let Err(err) = Hand::get_suit(card) {
      eprintln!("{}", err);
      return false
    };
    if let Err(err) = Hand::get_rank(card) {
      eprintln!("{}", err);
      return false
    };
    true
  }

  fn get_suit(card: &str) -> Result<String, String> {
    let suit_char = &card[(card.len() - 1)..card.len()].to_uppercase();

    match suit_char {
      x if (x == "S" || x == "H" || x == "C" || x == "D") => Ok(x.clone()),
      _ => Err(format!("{} {}", card, ERROR_MSGS[2]))
    }
  }

  fn get_rank(card: &str) -> Result<u32, String> {
    let rank = &card[0..(card.len()-1)];
    rank.parse::<u32>().or_else(|_| {
      match rank.to_uppercase() {
        r if (r == "A") => Ok(1),
        r if (r == "J") => Ok(11),
        r if (r == "Q") => Ok(12),
        r if (r == "K") => Ok(13),
        _ => Err(format!("{} {}", card, ERROR_MSGS[1]))
      }
    })
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
