/// Yields each item of a and then each item of b
pub fn append<I, J>(_a: I, _b: J) -> impl Iterator<Item = I::Item>
where
	I: Iterator,
	J: Iterator<Item = I::Item>,
{
	let mut result: Vec<I::Item> = vec![];
	for i in _a {
		result.push(i);
	}

	for i in _b {
		result.push(i);
	}

	result.into_iter()
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(_nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
	I: Iterator,
	I::Item: Iterator,
{
	let mut result: Vec<<I::Item as Iterator>::Item> = vec![];

	for inner in _nested_iter {
		for i in inner {
			result.push(i);
		}
	}

	result.into_iter()
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(_iter: I, _predicate: F) -> impl Iterator<Item = I::Item>
where
	I: Iterator,
	F: Fn(&I::Item) -> bool,
{
	let mut result: Vec<I::Item> = vec![];
	for i in _iter {
		if _predicate(&i) {
			result.push(i);
		}
	}

	result.into_iter()
}

pub fn length<I: Iterator>(_iter: I) -> usize {
	_iter.count()
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(_iter: I, _function: F) -> impl Iterator<Item = U>
where
	I: Iterator,
	F: Fn(I::Item) -> U,
{
	let mut result: Vec<U> = vec![];

	for i in _iter {
		result.push(_function(i));
	}

	result.into_iter()
}

pub fn foldl<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
	I: Iterator,
	F: Fn(U, I::Item) -> U,
{
	let mut result = _initial;

	for i in _iter {
		result = _function(result, i);
	}

	result
}

pub fn foldr<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
	I: DoubleEndedIterator,
	F: Fn(U, I::Item) -> U,
{
	let mut result = _initial;

	while let Some(i) = _iter.next_back() {
		result = _function(result, i);
	}

	result
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(mut _iter: I) -> impl Iterator<Item = I::Item> {
	let mut result: Vec<I::Item> = vec![];

	while let Some(i) = _iter.next_back() {
		result.push(i);
	}

	result.into_iter()
}
