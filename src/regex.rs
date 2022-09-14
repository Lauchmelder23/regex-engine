use crate::elements::{term::Term, element::Element};

#[derive(Debug)]
pub struct Regex {
	pub pattern: Vec<Term>
}

impl Regex {
	pub fn new() -> Regex {
		Regex { pattern: vec![] }
	}

	pub fn test(&self, text: &str) -> bool {
		for i in 0..text.len() {
			let mut it = text.split_at(i).1.chars();

			let mut matches = true;
			for atom in &self.pattern {
				if !atom.test(it.by_ref()) {
					matches = false;
					break;
				}
			}

			if matches {
				return true;
			}
		}	

		false
	}
}