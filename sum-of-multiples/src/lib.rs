pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
  (1..limit)
    .filter(|num| factors.iter().filter(|f| **f > 0).any(|f| num % f == 0))
    .fold(0, |mem, x| mem + x)
}
