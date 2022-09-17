#[macro_export]
macro_rules! assert_regex_eq {
	($regex: ident, $input: literal) => { assert_eq!($regex.test($input), $input.len()); }
}

#[macro_export]
macro_rules! assert_regex_ne {
	($regex: ident, $input: literal) => { assert_ne!($regex.test($input), $input.len()); }
}

pub fn setup(pattern: &str) -> regex::node::Branch {
	let res = regex::new(pattern);
	assert!(res.is_ok());

	return res.unwrap();
}