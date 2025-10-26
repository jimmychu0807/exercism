use std::{
	borrow::Borrow,
	io::{Error, Read, Write},
};

/// A munger which XORs a key with some data
#[derive(Debug, Clone)]
pub struct Xorcism<'a> {
	key: &'a [u8],
	cursor: usize,
}

pub struct XorcismReader<'a, R: Read> {
	xorcism: Xorcism<'a>,
	src: R,
}

impl<'a, R: Read> Read for XorcismReader<'a, R> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
		// read one byte from self.reader
		let mut buffer = [0; 1];
		let size = self.src.read(&mut buffer)?;

		if size == 0 {
			return Ok(0);
		}

		// xor with self.xorcism
		if let Some(byte) = self.xorcism.munge(buffer).next() {
			// writing the byte to the provided buffer
			buf[0] = byte;
			Ok(1)
		} else {
			Ok(0)
		}
	}
}

pub struct XorcismWriter<'a, W: Write> {
	xorcism: Xorcism<'a>,
	dest: W,
}

impl<'a, W: Write> Write for XorcismWriter<'a, W> {
	fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
		let mut size = 0;
		for byte in self.xorcism.munge(buf) {
			size += self.dest.write(&[byte])?;
		}

		Ok(size)
	}

	fn flush(&mut self) -> Result<(), Error> {
		self.dest.flush()
	}
}

impl<'a> Xorcism<'a> {
	/// Create a new Xorcism munger from a key
	///
	/// Should accept anything which has a cheap conversion to a byte slice.
	pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
		Xorcism { key: key.as_ref(), cursor: 0 }
	}

	pub fn reader(self, r: impl Read) -> impl Read {
		XorcismReader { xorcism: self, src: r }
	}

	pub fn writer(self, w: impl Write) -> impl Write {
		XorcismWriter { xorcism: self, dest: w }
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
			*data_byte ^= *key_byte;
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
	pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
	where
		Data: IntoIterator,
		Data::Item: Borrow<u8>,
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
