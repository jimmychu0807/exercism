use std::{
	// cell::RefCell,
	ptr::NonNull,
	marker::PhantomData,
	// rc::Rc
};

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

type Link<T> = Option<NonNull<Node<T>>>;

pub struct LinkedList<T> {
	head: Link<T>,
	tail: Link<T>,
	count: usize,
}

struct Node<T> {
	data: T,
	next: Link<T>,
	prev: Link<T>,
}

#[derive(PartialEq, Eq)]
enum CursorType {
	Front,
	Back,
}

pub struct Cursor<'a, T> {
	list: &'a mut LinkedList<T>,
	current: Link<T>,
	t: CursorType,
}

pub struct Iter<'a, T: 'a> {
	head: Link<T>,
	marker: PhantomData<&'a Node<T>>,
}

impl<T> LinkedList<T> {
	pub fn new() -> Self {
		Self { head: None, tail: None, count: 0 }
	}

	// You may be wondering why it's necessary to have is_empty()
	// when it can easily be determined from len().
	// It's good custom to have both because len() can be expensive for some types,
	// whereas is_empty() is almost always cheap.
	// (Also ask yourself whether len() is expensive for LinkedList)
	pub fn is_empty(&self) -> bool {
		self.count == 0
	}

	pub fn len(&self) -> usize {
		self.count
	}

	/// Return a cursor positioned on the front element
	pub fn cursor_front(&mut self) -> Cursor<'_, T> {
		let head = self.head;
		Cursor { list: self, current: head, t: CursorType::Front }
	}

	/// Return a cursor positioned on the back element
	pub fn cursor_back(&mut self) -> Cursor<'_, T> {
		let tail = self.tail;
		Cursor { list: self, current: tail, t: CursorType::Back }
	}

	/// Return an iterator that moves from front to back
	pub fn iter(&self) -> Iter<'_, T> {
		Iter {
			head: self.head,
			marker: PhantomData,
		}
	}
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
	/// Take a mutable reference to the current element
	pub fn peek_mut(&mut self) -> Option<&mut T> {
		todo!()
	}

	/// Move one position forward (towards the back) and
	/// return a reference to the new position
	#[allow(clippy::should_implement_trait)]
	pub fn next(&mut self) -> Option<&mut T> {
		todo!()
	}

	/// Move one position backward (towards the front) and
	/// return a reference to the new position
	pub fn prev(&mut self) -> Option<&mut T> {
		todo!()
	}

	/// Remove and return the element at the current position and move the cursor
	/// to the neighboring element that's closest to the back. This can be
	/// either the next or previous position.
	pub fn take(&mut self) -> Option<T> {
		let mut cur_ptr = self.current?;
		let item;

		unsafe {
			// handle prev node of the cursor, if exist
			if let Some(mut prev_node) = cur_ptr.as_mut().prev {
				prev_node.as_mut().next = cur_ptr.as_ref().next;
			}

			// handle next node of the cursor, if exist
			if let Some(mut next_node) = cur_ptr.as_mut().next {
				next_node.as_mut().prev = cur_ptr.as_ref().prev;
			}

			// this is the only node in the list
			if self.list.count == 1 {
				self.list.head = None;
				self.list.tail = None;
			} else if self.t == CursorType::Back && self.current == self.list.tail {
				// Need to adjust the list.tail ptr
				self.list.tail = cur_ptr.as_ref().prev;
			} else if self.t == CursorType::Front && self.current == self.list.head {
				// Need to adjust the list.head ptr
				self.list.head = cur_ptr.as_ref().next;
			}

			// extract the data out from cur_ptr to `item` var
			let boxed = Box::from_raw(cur_ptr.as_ptr());
			item = boxed.data;

			// then update self.current to prev_node
			self.current = match self.t {
				CursorType::Back => cur_ptr.as_ref().prev,
				CursorType::Front => cur_ptr.as_ref().next,
			};
		}

		self.list.count -= 1;
		Some(item)
	}

	pub fn insert_after(&mut self, _element: T) {
		// To make Node allocated in the heap
		let boxed = Box::new(Node { data: _element, next: None, prev: None });
		let mut new_ptr = NonNull::new(Box::into_raw(boxed)).unwrap();

		match self.current {
			None => {
				// this is an empty list
				self.list.head = Some(new_ptr);
				self.list.tail = Some(new_ptr);
				self.current = Some(new_ptr);
			}
			Some(mut cur_ptr) => unsafe {
				let cur = cur_ptr.as_mut();

				// wire new node
				new_ptr.as_mut().prev = Some(cur_ptr);
				new_ptr.as_mut().next = cur.next;

				cur.next = Some(new_ptr);

				if let Some(mut next_ptr) = new_ptr.as_ref().next {
					// This is not the last node, update the prev of the next node
					next_ptr.as_mut().prev = Some(new_ptr);
				} else {
					// This is the last node, update self.list tail
					self.list.tail = Some(new_ptr);
				}

				self.current = Some(new_ptr);
			},
		}
		self.list.count += 1;
	}

	pub fn insert_before(&mut self, _element: T) {
		let boxed = Box::new(Node { data: _element, next: None, prev: None });
		let mut new_ptr = NonNull::new(Box::into_raw(boxed)).unwrap();

		match self.current {
			None => {
				// this is an empty list
				self.list.head = Some(new_ptr);
				self.list.tail = Some(new_ptr);
				self.current = Some(new_ptr);
			}
			Some(mut cur_ptr) => unsafe {
				let cur = cur_ptr.as_mut();

				// wire new node
				new_ptr.as_mut().next = Some(cur_ptr);
				new_ptr.as_mut().prev = cur.prev;

				cur.prev = Some(new_ptr);

				if let Some(mut prev_ptr) = new_ptr.as_ref().prev {
					// there is a prev node
					prev_ptr.as_mut().next = Some(new_ptr);
				} else {
					// This is the head node, update self.list head
					self.list.head = Some(new_ptr);
				}

				self.current = Some(new_ptr);
			},
		}
		self.list.count += 1;
	}
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<&'a T> {
		// retrieve the item pointed by self.head
		let cur_ptr = self.head?;

		let item;
		unsafe {
			item = &cur_ptr.as_ref().data;

			// move self.head one step forward
			self.head = cur_ptr.as_ref().next;
		}

		// return the item
		Some(item)
	}
}
