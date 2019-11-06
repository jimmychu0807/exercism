use std::iter::FromIterator;
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Debug)]
struct Node<T: Debug> {
  data: T,
  next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct SimpleLinkedList<T: Debug> {
  head: Option<Box<Node<T>>>,
  size: usize,
}

impl<T: Debug> SimpleLinkedList<T> {
  pub fn new() -> Self {
    SimpleLinkedList { head: None, size: 0 }
  }

  pub fn len(&self) -> usize {
    self.size
  }

  pub fn push(&mut self, _element: T) {
    let head_box = self.head.take();
    self.head = Some(Box::new(Node { data: _element, next: head_box }));
    self.size += 1;
  }

  pub fn pop(&mut self) -> Option<T> {
    let head_box_opt = self.head.take();
    if head_box_opt.is_none() { return None }

    let head_box = head_box_opt.unwrap();
    self.head = head_box.next;
    self.size -= 1;
    Some(head_box.data)
  }

  pub fn peek(&self) -> Option<&T> {
    if self.head.is_none() { return None }

    let node_b_r = self.head.as_ref().unwrap();
    let node_r = node_b_r.deref();
    Some(&node_r.data)
  }

  pub fn rev(mut self) -> SimpleLinkedList<T> {
    let mut list = SimpleLinkedList::<T>::new();
    while self.peek().is_some() {
      list.push(self.pop().unwrap());
    }
    list
  }
}

impl<T: Debug> FromIterator<T> for SimpleLinkedList<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    let mut list = SimpleLinkedList::<T>::new();
    for item in iter { list.push(item) }
    list
  }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Debug> Into<Vec<T>> for SimpleLinkedList<T> {
  fn into(self) -> Vec<T> {
    let mut rev_list = self.rev();
    let mut vec = vec![];

    while rev_list.peek().is_some() {
      vec.push(rev_list.pop().unwrap());
    }
    vec
  }
}
