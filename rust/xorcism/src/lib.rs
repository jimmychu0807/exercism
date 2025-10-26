use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Debug, Clone)]
pub struct Xorcism<'a> {
	key: &'a [u8],
	cursor: usize,
}

impl<'a> Xorcism<'a> {
	/// Create a new Xorcism munger from a key
	///
	/// Should accept anything which has a cheap conversion to a byte slice.
	pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
		Xorcism { key: key.as_ref(), cursor: 0 }
	}

	/// XOR each byte of the input buffer with a byte from the key.
	///
	/// Note that this is stateful: repeated calls are likely to produce different results,
	/// even with identical inputs.
	pub fn munge_in_place(&mut self, data: &mut [u8]) {
		// Make the iterator repeats itself
		let mut key_iter = self.key.iter().cycle();

		// move key_iter to the cursor pos
		for _ in 0..self.cursor {
			let _ = key_iter.next();
		}

		for (data_byte, key_byte) in data.iter_mut().zip(key_iter) {
			*data_byte = *data_byte ^ *key_byte;
			self.cursor = (self.cursor + 1) % self.key.len();
		}
	}

	/// XOR each byte of the data with a byte from the key.
	///
	/// Note that this is stateful: repeated calls are likely to produce different results,
	/// even with identical inputs.
	///
	/// Should accept anything which has a cheap conversion to a byte iterator.
	/// Shouldn't matter whether the byte iterator's values are owned or borrowed.
	pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8> where
		Data: IntoIterator,
		Data::Item: Borrow<u8>
	{
		// Make the iterator repeats itself
		let mut key_iter = self.key.iter().cycle();

		// move key_iter to the cursor pos
		for _ in 0..self.cursor {
			let _ = key_iter.next();
		}

		let res_vec = data
			.into_iter()
			.zip(key_iter)
			.map(|(data_byte, key_byte)| {
				let db: &u8 = data_byte.borrow();
				let res = *db ^ *key_byte;
				self.cursor = (self.cursor + 1) % self.key.len();
				res
			})
			.collect::<Vec<_>>();

		res_vec.into_iter()
	}
}
