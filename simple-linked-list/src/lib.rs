use std::rc::Rc;
use std::iter::FromIterator;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T: Debug> {
  data: T,
  next: Option<Rc<Node<T>>>,
}

impl<T: Debug> Node<T> {
  fn new(obj: T) -> Self {
    Node { data: obj, next: None }
  }
}

#[derive(Debug)]
pub struct SimpleLinkedList<T: Debug> {
  head: Option<Rc<Node<T>>>,
  tail: Option<Rc<Node<T>>>,
}

impl<T: Debug> SimpleLinkedList<T> {
  pub fn new() -> Self {
    SimpleLinkedList { head: None, tail: None }
  }

  pub fn len(&self) -> usize {
    let mut node = &self.head;
    let mut llen: usize = 0;

    while node.is_some() {
      llen += 1;
      // WHY: this work?
      node = &node.as_ref().unwrap().next;
    }
    llen
  }

  pub fn push(&mut self, _element: T) {
    let node_rc = Rc::new(Node::new(_element));
    if self.head.is_none() {
      self.head = Some(Rc::clone(&node_rc));
      self.tail = Some(Rc::clone(&node_rc));
    } else {
      if Rc::ptr_eq(self.head.as_ref().unwrap(), self.tail.as_ref().unwrap()) {
        // println!("before: count: {}, obj: {:?}", Rc::strong_count(head_rc), head_rc);
        // drop(self.tail.as_mut().unwrap());
        // println!("after: count: {}, obj: {:?}", Rc::strong_count(head_rc), head_rc);
        self.tail = None;
        let head_rc = self.head.as_mut().unwrap();
        let mut head_node = Rc::get_mut(head_rc).unwrap();
        head_node.next = Some(Rc::clone(&node_rc));
        self.tail = Some(Rc::clone(&node_rc));
      } else {
        let node_ptr = Rc::into_raw(*self.tail.as_mut().unwrap());
        let mut tail_node: Node<T> = (*node_ptr).into();
        // let mut tail_node = Rc::get_mut(&mut tail_rc).unwrap();
        tail_node.next = Some(Rc::clone(&node_rc));
        self.tail = Some(Rc::clone(&node_rc));
      }
    }
  }

  // pub fn pop(&mut self) -> Option<T> {
  //     unimplemented!()
  // }

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
