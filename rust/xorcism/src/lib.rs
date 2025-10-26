/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
	key: &'a [u8],
}

// pub trait AsBytes {
// 	fn as_bytes(&self) -> &[u8];
// }

// impl AsBytes for &[u8] {
// 	fn as_bytes(&self) -> &[u8] {
// 		self.iter().map(|v| v.as_bytes()).collect::<Vec<_>>()
// 	}
// }

impl<'a> Xorcism<'a> {
	/// Create a new Xorcism munger from a key
	///
	/// Should accept anything which has a cheap conversion to a byte slice.
	pub fn new<Key: AsRef<[u8]>>(key: &'a Key) -> Xorcism<'a> {
		Xorcism {
			key: key.as_ref()
		}
	}

	/// XOR each byte of the input buffer with a byte from the key.
	///
	/// Note that this is stateful: repeated calls are likely to produce different results,
	/// even with identical inputs.
	pub fn munge_in_place(&mut self, data: &mut [u8]) {
		todo!()
	}

	/// XOR each byte of the data with a byte from the key.
	///
	/// Note that this is stateful: repeated calls are likely to produce different results,
	/// even with identical inputs.
	///
	/// Should accept anything which has a cheap conversion to a byte iterator.
	/// Shouldn't matter whether the byte iterator's values are owned or borrowed.
	pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8> {
		todo!();
		// this empty iterator silences a compiler complaint that
		// () doesn't implement ExactSizeIterator
		std::iter::empty()
	}
}
