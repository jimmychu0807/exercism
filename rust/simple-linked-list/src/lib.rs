pub struct Node<T> {
	data: T,
	next: Option<Box<Self>>,
}

pub struct SimpleLinkedList<T> {
	head: Option<Box<Node<T>>>,
	len: usize,
}

impl<T> SimpleLinkedList<T> {
	pub fn new() -> Self {
		Self { head: None, len: 0 }
	}

	// You may be wondering why it's necessary to have is_empty()
	// when it can easily be determined from len().
	// It's good custom to have both because len() can be expensive for some types,
	// whereas is_empty() is almost always cheap.
	// (Also ask yourself whether len() is expensive for SimpleLinkedList)
	pub fn is_empty(&self) -> bool {
		self.head.is_none()
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn push(&mut self, _element: T) {
		let new_node = Node { data: _element, next: self.head.take() };

		self.head = Some(Box::new(new_node));
		self.len += 1;
	}

	pub fn pop(&mut self) -> Option<T> {
		match self.head.take() {
			None => None,
			Some(mut node) => {
				self.head = node.next.take();
				self.len -= 1;
				Some(node.data)
			}
		}
	}

	pub fn peek(&self) -> Option<&T> {
		match &self.head {
			None => None,
			Some(node) => Some(&node.data),
		}
	}

	#[must_use]
	pub fn rev(mut self) -> SimpleLinkedList<T> {
		let mut ll = Self::new();

		while let Some(data) = self.pop() {
			ll.push(data);
		}

		ll
	}
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
	fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
		let mut ll = Self::new();

		for data in _iter.into_iter() {
			ll.push(data);
		}

		ll
	}
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
	fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
		let mut vec: Vec<T> = vec![];

		let mut rev = _linked_list.rev();
		while let Some(data) = rev.pop() {
			vec.push(data);
		}

		vec
	}
}
