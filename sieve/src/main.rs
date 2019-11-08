use sieve;

fn main() {
  let vec1 = sieve::primes_up_to(100);
  println!("{:?}", vec1);
  let vec2 = sieve::primes_up_to2(100);
  println!("{:?}", vec2);
  assert_eq!(vec1, vec2);
}
