use std::{
	collections::{HashSet, hash_set::Iter},
	hash::Hash,
};

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: Eq + Hash> {
	inner: HashSet<T>,
}

impl<T> CustomSet<T>
where
	T: Eq + Hash + Clone,
{
	pub fn new(_input: &[T]) -> Self {
		let mut set = Self { inner: HashSet::<T>::new() };

		for val in _input {
			set.inner.insert(val.clone());
		}

		set
	}

	pub fn contains(&self, _element: &T) -> bool {
		self.inner.contains(_element)
	}

	pub fn add(&mut self, _element: T) {
		self.inner.insert(_element);
	}

	pub fn is_subset(&self, _other: &Self) -> bool {
		self.inner.iter().all(|v| _other.contains(v))
	}

	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}

	pub fn is_disjoint(&self, _other: &Self) -> bool {
		self.inner.iter().all(|v| !_other.contains(v))
	}

	#[must_use]
	pub fn intersection(&self, _other: &Self) -> Self {
		let val_vec = self.inner.iter().filter(|v| _other.contains(v)).cloned().collect::<Vec<_>>();

		CustomSet::<T>::new(&val_vec)
	}

	#[must_use]
	pub fn difference(&self, _other: &Self) -> Self {
		let val_vec =
			self.inner.iter().filter(|v| !_other.contains(v)).cloned().collect::<Vec<_>>();

		CustomSet::<T>::new(&val_vec)
	}

	pub fn iter(&self) -> Iter<T> {
		self.inner.iter()
	}

	#[must_use]
	pub fn union(&self, _other: &Self) -> Self {
		let mut set = Self::new(&self.inner.iter().cloned().collect::<Vec<_>>());

		for val in _other.iter() {
			set.add(val.clone());
		}

		set
	}
}
