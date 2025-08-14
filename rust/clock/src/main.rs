use clock::Clock;

fn main() {
	println!("{}", Clock::new(0, 0));
	println!("{}", Clock::new(0, 1));
	println!("{}", Clock::new(0, -1));
	println!("{}", Clock::new(0, 60));
	println!("{}", Clock::new(0, 121));
	println!("{}", Clock::new(-1, 0));
	println!("{}", Clock::new(24, 0));
	println!("{}", Clock::new(49, 0));
}
