use crate::elements::{alternative::Alternative, element::Element};

#[derive(Debug)]
pub struct Regex {
	pub pattern: Alternative
}

impl Regex {
	pub fn new(regexp: &str) -> Regex {
		Regex { pattern: <Alternative as Element>::parse(regexp, 0).unwrap().0 }
	}

	pub fn test(&self, text: &str) -> bool {
		for i in 0..text.len() {
			let mut it = text.split_at(i).1.chars();

			if self.pattern.test(it.by_ref()) {
				return true;
			}
		}

		false
	}
}