use poker::{self, PokerHand, PokerHandPattern};

fn main() {
  let poker1 = PokerHand::new("4S 5S 7H 8D JC");
  println!("{:?}", poker1);
}
