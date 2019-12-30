// use poker::{self, PokerHand};
use poker::{ self, Hand as PokerHand };
use core::cmp::{ Ordering };

fn main() -> Result<(), String> {
  let poker1 = PokerHand::new("4S 5S 7H 8D JC")?;
  println!("{:?}", poker1);

  // let poker1a = PokerHand::new("4S 5S 6H 8D JC");
  // let poker1b = PokerHand::new("4S 5S 6H 9D JC");

  // let poker2 = PokerHand::new("4D 5D 7D 8D 6D");
  // println!("{:?}", poker2);

  // let poker3 = PokerHand::new("10D JD QD KD AD");
  // println!("{:?}", poker3);

  // let poker4 = PokerHand::new("2D JD QD KD AD");
  // println!("{:?}", poker4);

  // let poker5 = PokerHand::new("2S 2H 2C 2D AD");
  // println!("{:?}", poker5);

  // let poker6 = PokerHand::new("2S 2H 2C 5D 5C");
  // println!("{:?}", poker6);

  // let poker7 = PokerHand::new("2S 2H 2C 3D 4D");
  // println!("{:?}", poker7);

  // let poker8 = PokerHand::new("2S 2H 3C 3D 4D");
  // println!("{:?}", poker8);

  // let poker9 = PokerHand::new("2S 2H 3C 4D 5D");
  // println!("{:?}", poker9);

  // assert_eq!(poker1.eq(&poker1), true);
  // assert_eq!(poker1.partial_cmp(&poker1a), Some(Ordering::Greater));
  // assert_eq!(poker1.partial_cmp(&poker1b), Some(Ordering::Less));
  Ok(())
}
