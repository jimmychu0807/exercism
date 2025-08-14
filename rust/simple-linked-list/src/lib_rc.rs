use std::rc::Rc;
use std::cell::{RefCell, RefMut};
// use std::iter::FromIterator;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T: Debug> {
  data: T,
  next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug> Node<T> {
  fn new(obj: T) -> Self {
    Node { data: obj, next: None }
  }
}

#[derive(Debug)]
pub struct SimpleLinkedList<T: Debug> {
  head: Option<Rc<RefCell<Node<T>>>>,
  tail: Option<Rc<RefCell<Node<T>>>>,
  len: usize,
}

impl<T: Debug> SimpleLinkedList<T> {
  pub fn new() -> Self {
    SimpleLinkedList { head: None, tail: None, len: 0 }
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn push(&mut self, _element: T) {
    let node_rc = Rc::new(RefCell::new(Node::new(_element)));
    self.len += 1;

    match self.head {
      None => { self.head = Some(Rc::clone(&node_rc)); },
      Some(_) => {
        let mut node: RefMut<_> = self.tail.as_ref().unwrap().borrow_mut();
        node.next = Some(Rc::clone(&node_rc));
      }
    }
    self.tail = Some(Rc::clone(&node_rc));
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.head.is_none() { return None }
    self.len -= 1;

    // single node left in the linked list
    if self.len == 0 {
      let node = self.tail.as_ref().unwrap().into_inner();
      self.head = None;
      self.tail = None;
      return Some(node.data);
    }

    let mut curr_rc = self.head.as_ref().unwrap();
    let tail_node_rc = self.tail.as_ref().unwrap();
    while !Rc::ptr_eq(curr_rc.borrow().next.as_ref().unwrap(), tail_node_rc) {
      curr_rc = curr_rc.borrow().next.as_ref().unwrap();
    }
    let val = curr_rc.borrow().next.as_ref().unwrap().into_inner();
    curr_rc.borrow_mut().next = None;
    self.tail = Some(Rc::clone(&curr_rc));
    Some(val.data)
  }

  // pub fn peek(&self) -> Option<&T> {
  //     unimplemented!()
  // }

  // pub fn rev(self) -> SimpleLinkedList<T> {
  //     unimplemented!()
  // }
}

// impl<T> FromIterator<T> for SimpleLinkedList<T> {
//   fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
//     unimplemented!()
//   }
// }

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
        unimplemented!()
    }
}
