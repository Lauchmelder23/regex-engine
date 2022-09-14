use crate::{
	regex::Regex,
	elements::{
		term::Term, element::Element
	}
};

pub fn parse(regexp: &str) -> Regex {
	let mut regex = Regex::new();

	let mut pos = 0;
	while pos < regexp.len() {
		pos = match <Term as Element>::parse(regexp, pos) {
			Some((term, new_pos)) => {
				regex.pattern.push(term);
				new_pos
			},
			None => break
		};
	}

	regex
}