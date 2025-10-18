use std::{
	cmp::{PartialEq, PartialOrd},
	fmt::Debug,
};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Tree<T: Debug + Ord> {
	label: T,
	children: Vec<Box<Self>>,
}

impl<T: Debug + Ord> Tree<T> {
	pub fn new(label: T) -> Self {
		Self { label, children: Vec::new() }
	}

	/// Builder-method for constructing a tree with children
	pub fn with_child(mut self, child: Self) -> Self {
		self.children.insert(
			self.children.binary_search_by(|c| c.label.cmp(&child.label)).unwrap_err(),
			Box::new(child),
		);

		self
	}

	pub fn pov_from(&mut self, target: &T) -> bool {
		self.pov_from_rec(target).is_some()
	}

	// Returns (tree rooted at 'target', remainder of parent's children)
	fn pov_from_rec(&mut self, target: &T) -> Option<Vec<usize>> {
		if self.label == *target {
			return Some(Vec::new());
		}

		// find the path to the target
		let (idx, mut idx_list) =
			self.children.iter_mut().enumerate().find_map(|(idx, child)| {
				child.pov_from_rec(target).map(|idx_list| (idx, idx_list))
			})?;

		// cut the branch out from the tree.
		let mut old_pov = self.children.remove(idx);
		std::mem::swap(self, &mut old_pov);

		// now traverse down the newly formed tree to find the node where the insert happens
		let mut insertion_node = self;
		for i in idx_list.iter() {
			insertion_node = insertion_node.children[*i].as_mut();
		}

		// add the old_pov as the child of the insertion node
		let insert_pos =
			insertion_node.children.binary_search_by(|c| c.label.cmp(&old_pov.label)).unwrap_err();
		insertion_node.children.insert(insert_pos, old_pov);

		idx_list.push(insert_pos);

		// the idx_list stores how to traverse back down the new tree to the insertion node
		Some(idx_list)
	}

	pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
		if !self.pov_from(from) {
			return None;
		}

		self.path_to_rec(to)
	}

	fn path_to_rec(&self, to: &T) -> Option<Vec<&T>> {
		if self.label == *to {
			return Some(vec![&self.label]);
		}

		let mut path = self.children.iter().find_map(|child| child.path_to_rec(to))?;

		path.insert(0, &self.label);

		Some(path)
	}
}
