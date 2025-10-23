use std::fmt::Debug;

#[derive(Debug)]
pub struct CircularBuffer<T> {
	data: Vec<Option<T>>,
	next: usize, // next insertion slot
	cursor: Option<usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	EmptyBuffer,
	FullBuffer,
}

impl<T> CircularBuffer<T>
where
	T: Clone + Debug,
{
	pub fn new(capacity: usize) -> Self {
		Self { data: vec![None; capacity], next: 0, cursor: None }
	}

	fn is_full(&self) -> bool {
		if let Some(pos) = self.cursor {
			if pos == self.next {
				return true;
			}
		}

		false
	}

	pub fn write(&mut self, _element: T) -> Result<(), Error> {
		let capacity = self.data.len();

		// check if next == cursor content.
		// if yes, return error
		if self.is_full() {
			return Err(Error::FullBuffer);
		}

		// write the content to vec
		self.data[self.next] = Some(_element);

		// if cursor is None, set cursor to this position
		if self.cursor.is_none() {
			self.cursor = Some(self.next);
		}

		// increment next (mod capacity)
		self.next = (self.next + 1) % capacity;

		Ok(())
	}

	pub fn read(&mut self) -> Result<T, Error> {
		let capacity = self.data.len();

		let Some(mut cursor_pos) = self.cursor else {
			return Err(Error::EmptyBuffer);
		};

		let value = self.data[cursor_pos].clone().unwrap();

		// increment cursor by 1
		cursor_pos = (cursor_pos + 1) % capacity;

		// if cursor == next, remove cursor
		if cursor_pos != self.next {
			self.cursor = Some(cursor_pos);
		} else {
			self.cursor = None;
		}

		// return value
		Ok(value)
	}

	pub fn clear(&mut self) {
		self.data = vec![None; self.data.len()];
		self.next = 0;
		self.cursor = None;
	}

	pub fn overwrite(&mut self, _element: T) {
		if !self.is_full() {
			let _ = self.write(_element);
			return;
		}

		// the circular buffer is full
		let capacity = self.data.len();

		// write the content to vec
		self.data[self.next] = Some(_element);

		// increment next (mod capacity)
		self.next = (self.next + 1) % capacity;
		self.cursor = Some(self.next);
	}
}
