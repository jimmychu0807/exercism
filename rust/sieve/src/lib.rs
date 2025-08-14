use std::collections::BTreeSet;
use std::convert::TryInto;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
  primes_up_to1(upper_bound)
}

pub fn primes_up_to1(upper_bound: u64) -> Vec<u64> {
  if upper_bound < 2 { return vec![] }
  let mut primes = (2..=upper_bound).collect::<BTreeSet<_>>();
  let mut pos = 0;
  while pos < primes.len() {
    let el = primes.iter().nth(pos).unwrap().clone();
    let multiples = get_multiples(&el, &upper_bound);
    primes = primes.difference(&multiples).map(|n| *n).collect();
    pos += 1;
  }
  primes.into_iter().collect::<Vec<u64>>()
}

fn get_multiples(el: &u64, upper_bound: &u64) -> BTreeSet<u64> {
  (2..=*upper_bound)
    .into_iter()
    .filter_map(|n| match n {
      n if n*(*el) <= *upper_bound => Some(n*(*el)),
      _ => None
    })
    .collect::<BTreeSet<u64>>()
}

pub fn primes_up_to2(upper_bound: u64) -> Vec<u64> {
  let ub_usize: usize = upper_bound.try_into().unwrap();
  let mut marking = vec![false; ub_usize + 1];
  for i in 2..=ub_usize {
    if marking[i] { continue }
    swipe_true(&mut marking, i as u64);
  }
  marking
    .into_iter()
    .enumerate()
    .filter_map(|(ind, val)| if ind > 1 && !val { Some(ind as u64) } else { None })
    .collect::<Vec<u64>>()
}

fn swipe_true(marking: &mut Vec<bool>, el: u64) {
  let el_usize = el as usize;
  let marking_len = marking.len();
  (2..marking_len)
    .into_iter()
    .filter_map(|n| if n * el_usize <= marking_len { Some(n * el_usize) } else { None })
    .for_each(|n| marking[n] = true);
}
