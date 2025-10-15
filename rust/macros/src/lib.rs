#[macro_export]
macro_rules! hashmap {
	() => {
		{
			let mut map = ::std::collections::HashMap::new();
			map
		}

	};

	( $( $k:expr => $v:expr ),+ $(,)? ) => {
		{
			let mut map = ::std::collections::HashMap::new();
			$(
				map.insert($k, $v);
			)*

			map
		}
	};
}
