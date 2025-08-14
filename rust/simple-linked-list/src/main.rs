use simple_linked_list::SimpleLinkedList;

fn main() {
  let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
  list.push(1);
  println!("{:?}", list);
  list.push(2);
  println!("{:?}", list);
  list.push(3);
  println!("{:?}", list);
}
