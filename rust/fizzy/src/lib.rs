use std::ops::Rem;

pub struct Matcher<T> {
	pub func: fn(T) -> bool,
	pub out: String,
}

impl<T> Matcher<T> {
	pub fn new(func: fn(T) -> bool, out: &str) -> Self {
		Self { func, out: out.into() }
	}
}

pub struct Fizzy<T> {
	matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
	T: ToString + std::marker::Copy,
{
	pub fn new() -> Self {
		Self { matchers: vec![] }
	}

	pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
		self.matchers.push(matcher);
		self
	}

	pub fn apply<I>(&self, _iter: I) -> impl Iterator<Item = String>
	where
		I: Iterator<Item = T>,
	{
		_iter.map(|v| {
			let mut res: String = "".into();
			let mut converted = false;

			for matcher in self.matchers.iter() {
				if (matcher.func)(v) {
					converted = true;
					res += &matcher.out;
				}
			}

			if !converted {
				res = v.to_string();
			}

			res
		})
	}
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
	T: ToString + std::marker::Copy + Rem<Output = T> + From<u8> + PartialEq,
{
	Fizzy::<T>::new()
		.add_matcher(Matcher::new(|v: T| v.rem(3.into()) == 0.into(), "fizz"))
		.add_matcher(Matcher::new(|v: T| v.rem(5.into()) == 0.into(), "buzz"))
}
