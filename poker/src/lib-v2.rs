use core::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand<'a> {
  hand_str: &'a str,
  cards: Vec<&'a str>
}

#[derive(Debug, Clone)]
pub enum HandPattern {
  StraightFlush(u32),              // 同花順
  FourOfAKind(u32, u32),           // 4-1
  FullHouse(u32, u32),             // 3-2
  Flush(u32, u32, u32, u32, u32),  // 同花
  Straight(u32),                   // 順
  ThreeOfAKind(u32, u32, u32),     // 3-1-1
  TwoPair(u32, u32, u32),          // 2-2-1
  OnePair(u32, u32, u32, u32),     // 2-1-1-1
  Nothing(u32, u32, u32, u32, u32),
}

impl HandPattern {
  pub fn pattern_rank(&self) -> u32 {
    match self {
      HandPattern::StraightFlush(_) => 100,
      HandPattern::FourOfAKind(_, _) => 90,
      HandPattern::FullHouse(_, _) => 80,
      HandPattern::Flush(_, _, _, _, _) => 70,
      HandPattern::Straight(_) => 60,
      HandPattern::ThreeOfAKind(_, _, _) => 50,
      HandPattern::TwoPair(_, _, _) => 40,
      HandPattern::OnePair(_, _, _, _) => 30,
      HandPattern::Nothing(_, _, _, _, _) => 20,
    }
  }
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
    // TODO
    Ordering::Equal
  }

  // This method is for determining the Hand pattern
  pub fn pattern(&self) -> HandPattern {
    let mut rank_map: HashMap<u32, u32> = HashMap::new();
    let mut suit_map: HashMap<String, u32> = HashMap::new();

    self.cards.iter().map(|c| (
      Hand::get_rank(c).unwrap(),
      Hand::get_suit(c).unwrap()
    ))
      .for_each(|(rank, suit)| {
        if let Some(val) = rank_map.get_mut(&rank) {
           *val = *val + 1;
        } else {
          rank_map.insert(rank, 1);
        }
        if let Some(val) = suit_map.get_mut(&suit) {
          *val = *val + 1;
        } else {
          suit_map.insert(suit, 1);
        }
      });

    if rank_map.keys().count() == 5 && suit_map.keys().count() == 1 {
      let sorted_cards_rank = self.sorted_cards_rank();
      // 同花 or 同花順
      if self.is_straight() {
        return HandPattern::StraightFlush(sorted_cards_rank[0]);
      } else {
        return HandPattern::Flush(sorted_cards_rank[0], sorted_cards_rank[1], sorted_cards_rank[2],
          sorted_cards_rank[3], sorted_cards_rank[4]);
      }
    } else if rank_map.keys().count() == 2 {
      if let Some((four_key, _)) = rank_map.iter().filter(|(_, v)| **v == 4).nth(0) {
        let (one_key, _) = rank_map.iter().filter(|(_, v)| **v == 1).nth(0).unwrap();
        return HandPattern::FourOfAKind(*four_key, *one_key);
      } else {
        let (three_key, _) = rank_map.iter().filter(|(_, v)| **v == 3).nth(0).unwrap();
        let (two_key, _) = rank_map.iter().filter(|(_, v)| **v == 2).nth(0).unwrap();
        return HandPattern::FullHouse(*three_key, *two_key);
      }
    } else if rank_map.keys().count() == 3 {
      if let Some((three_key, _)) = rank_map.iter().filter(|(_, v)| **v == 3).nth(0) {
        let mut one_keys: Vec<u32> = rank_map.iter().filter_map(|(k, v)| if *v == 1 { Some(*k) } else { None }).collect();
        one_keys.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        return HandPattern::ThreeOfAKind(*three_key, one_keys[0], one_keys[1]);
      } else {
        let mut two_keys: Vec<u32> = rank_map.iter().filter_map(|(k, v)| if *v == 2 { Some(*k) } else { None }).collect();
        two_keys.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        let one_keys: Vec<u32> = rank_map.iter().filter_map(|(k, v)| if *v == 1 { Some(*k) } else { None }).collect();
        return HandPattern::TwoPair(two_keys[0], two_keys[1], one_keys[0]);
      }
    } else if rank_map.keys().count() == 4 {
        let two_keys: Vec<u32> = rank_map.iter().filter_map(|(k, v)| if *v == 2 { Some(*k) } else { None }).collect();
        let mut one_keys: Vec<u32> = rank_map.iter().filter_map(|(k, v)| if *v == 1 { Some(*k) } else { None }).collect();
        one_keys.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        return HandPattern::OnePair(two_keys[0], one_keys[0], one_keys[1], one_keys[2]);
    } else { // this is the condition satisfied: rank_map.keys().count() == 5
      let sorted_cards_rank = self.sorted_cards_rank();
      if self.is_straight() {
        // 順
        return HandPattern::Straight(sorted_cards_rank[0]);
      } else {
        return HandPattern::Nothing(sorted_cards_rank[0], sorted_cards_rank[1], sorted_cards_rank[2],
          sorted_cards_rank[3], sorted_cards_rank[4]);
      }
    }
  }

  fn is_straight(&self) -> bool {
    let sorted_cards_rank = self.sorted_cards_rank();
    for i in 0..4 {
      if sorted_cards_rank[i] - sorted_cards_rank[i+1] != 1 { return false; }
    }
    return true;
  }

  fn sorted_cards_rank(&self) -> Vec<u32> {
    let mut sorted_cards_rank = self.cards.iter().map(|c| Hand::get_rank(c).unwrap()).collect::<Vec<_>>();
    sorted_cards_rank.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    if sorted_cards_rank == vec![14, 5, 4, 3, 2] {
      // only in this condition, "A" is treated as 1
      sorted_cards_rank = vec![5, 4, 3, 2, 1];
    }
    return sorted_cards_rank;
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
        r if (r == "J") => Ok(11),
        r if (r == "Q") => Ok(12),
        r if (r == "K") => Ok(13),
        r if (r == "A") => Ok(14),
        _ => Err(format!("{} {}", card, ERROR_MSGS[1]))
      }
    })
  }
}

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
