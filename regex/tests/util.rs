use regex::regex::Regex;

#[macro_export]
macro_rules! assert_match {
	($regex: ident, $input: literal, $pos: literal) => { 
		let result = $regex.test($input);
		assert!(result.is_some());
		assert_eq!(result.unwrap(), $pos); 
	}
}

#[macro_export]
macro_rules! assert_failure {
	($regex: ident, $input: literal) => { 
		assert!($regex.test($input).is_none());
	}
}

pub fn setup(pattern: &str) -> Regex {
	let res = Regex::new(pattern);
	assert!(res.is_ok());

	return res.unwrap();
}