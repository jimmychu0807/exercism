use std::cmp::{min};
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Bucket {
  One,
  Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BucketStats {
  /// The total number of "moves" it should take to reach the desired number of liters, including
  /// the first fill.
  pub moves: u8,
  /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
  pub goal_bucket: Bucket,
  /// How many liters are left in the other bucket?
  pub other_bucket: u8,
}

pub type BucketState = ((u8, u8), (u8, u8));

pub fn get_solution(state: BucketState, goal: u8, moves: u8) -> Option<BucketStats> {
  let (bucket_1, bucket_2) = state;
  let goal_bucket = if bucket_1.0 == goal { Bucket::One } else { Bucket::Two };
  let other_bucket = if bucket_1.0 == goal { bucket_2.0 } else { bucket_1.0 };
  Some(BucketStats { moves, goal_bucket, other_bucket })
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {

  // initial case
  let curr_cap_1 = if *start_bucket == Bucket::One { capacity_1 } else { 0 };
  let curr_cap_2 = if *start_bucket == Bucket::Two { capacity_2 } else { 0 };
  let bucket_1 = (curr_cap_1, capacity_1);
  let bucket_2 = (curr_cap_2, capacity_2);

  if curr_cap_1 == goal || curr_cap_2 == goal {
    return get_solution((bucket_1, bucket_2), goal, 1);
  }

  let mut seen = HashSet::new();
  // empty state
  seen.insert(((0, capacity_1), (0, capacity_2)));
  // both initial states are marked as seen
  seen.insert(((0, capacity_1), (capacity_2, capacity_2)));
  seen.insert(((capacity_1, capacity_1), (0, capacity_2)));
  rec_solve((bucket_1, bucket_2), &mut seen, goal, 1)
}

pub fn rec_solve(state: BucketState, seen: &mut HashSet<BucketState>, goal: u8, moves: u8)
  -> Option<BucketStats> {

  let bucket_1 = state.0;
  let bucket_2 = state.1;

  // println!("moves: {:?}, state: {:?}", moves, state);
  // println!("seen: {:?}", seen);

  if bucket_1.0 == goal || bucket_2.0 == goal {
    // println!("found solution: {:?}", state);
    return get_solution((bucket_1, bucket_2), goal, moves);
  }

  // there are total of 6 possible ops on the two buckets
  let all_new_states = [
    // emptying or filling bucket_1
    ((0, bucket_1.1), bucket_2.clone()),
    ((bucket_1.1, bucket_1.1), bucket_2.clone()),
    // emptying or filling bucket_2
    (bucket_1.clone(), (0, bucket_2.1)),
    (bucket_1.clone(), (bucket_2.1, bucket_2.1)),
    // bucket_1 pouring to bucket_2
    ( (bucket_1.0 - min(bucket_1.0, bucket_2.1 - bucket_2.0), bucket_1.1),
      (bucket_2.0 + min(bucket_1.0, bucket_2.1 - bucket_2.0), bucket_2.1)),
    // bucket_2 pouring to bucket_1
    ( (bucket_1.0 + min(bucket_2.0, bucket_1.1 - bucket_1.0), bucket_1.1),
      (bucket_2.0 - min(bucket_2.0, bucket_1.1 - bucket_1.0), bucket_2.1)),
  ];

  let mut results: Vec<BucketStats> = all_new_states
    .into_iter()
    .filter_map(|new_state| match seen.contains(&new_state) {
      true => None,
      false => {
        // add in a new disable state, call the rec function
        seen.insert(new_state.clone());
        rec_solve(new_state.clone(), seen, goal, moves + 1)
      },
    }) // this returned `BucketStats`, not `Option<BucketStats>`. Feature of filter_map.
    .collect();

  // println!("results: {:?}", results);

  if results.len() == 0 { return None }
  // find the one with min `moves` and return
  results.sort_by(|s1, s2| s1.moves.partial_cmp(&s2.moves).unwrap());
  Some((*results.first().unwrap()).clone())
}
