fn is_prime(n: u32) -> bool {
  let upto = (n as f32).sqrt().ceil() as u32;
  let mut trial = 2;

  // It can only be 0, 1, 2
  if trial > upto { return true; }
  while trial <= upto {
    if n % trial == 0 { return false; }
    trial += if trial % 2 == 0 { 1 } else { 2 };
  }
  true
}

pub fn nth(n: u32) -> u32 {
  let mut prime_arr = vec![2];
  let mut trial = prime_arr.last().unwrap().clone();

  while n >= (prime_arr.len() as u32) {
    trial += 1;
    if !is_prime(trial) { continue; }
    prime_arr.push(trial);
  }
  return *prime_arr.last().unwrap();
}
