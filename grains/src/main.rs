use grains;

fn main() {
  println!("1: {:?}", grains::square(1));
  println!("2: {:?}", grains::square(2));
  println!("3: {:?}", grains::square(3));
  println!("4: {:?}", grains::square(4));
  println!("16: {:?}", grains::square(16));
  println!("64: {:?}", grains::square(64));
  // println!("0: {:?}", grains::square(0));
  println!("total: {:?}", grains::total());
}
