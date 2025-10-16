use std::{collections::HashSet, fmt::Debug, hash::Hash};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<'a, T: Debug + Ord + Hash> {
	root: Node<'a, T>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node<'a, T: Hash> {
	parent: Option<&'a Self>,
	val: T,
	children: HashSet<Self>,
}

impl<T: Debug + Ord> Tree<T> {
	pub fn new(label: T) -> Self {
		todo!("Create a new tree with the given {label:?}");
	}

	/// Builder-method for constructing a tree with children
	pub fn with_child(self, child: Self) -> Self {
		todo!("Add {child:?} to the tree and return the tree");
	}

	pub fn pov_from(&mut self, from: &T) -> bool {
		todo!("Reparent the tree with the label {from:?} as root");
	}

	pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
		todo!("Return the shortest path between {from:?} and {to:?}");
	}
}
