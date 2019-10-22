fn is_prime(n: &u32) -> bool {
  true
}

pub fn nth(n: u32) -> u32 {
  let mut prime_arr = vec![2, 3, 5, 7, 11, 13];
  let mut trial = prime_arr.last().unwrap().clone();

  while n < (prime_arr.len() as u32) {
    trial += 1;
    if !is_prime(&trial) { continue; }
    prime_arr.push(trial);
  }

  return *prime_arr.last().unwrap();
}
