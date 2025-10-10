pub fn map<T, U, F>(input: Vec<T>, mut function: F) -> Vec<U>
where
	F: FnMut(T) -> U,
{
	let mut res: Vec<U> = vec![];

	for val in input.into_iter() {
		res.push(function(val));
	}

	res
}
