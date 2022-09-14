use std::str::Chars;

use super::{term::Term, element::{Element, ParseResult}};

#[derive(Debug)]
pub struct Alternative {
	alternative: Option<Box<Alternative>>,
	term: Term
}

impl Element for Alternative {
	fn parse(expr: &str, pos: usize) -> ParseResult<Alternative> {
		match <Term as Element>::parse(expr, pos) {
			None => None,
			Some((term, new_pos)) => match <Alternative as Element>::parse(expr, new_pos) {
				None => Some((
					Alternative {
						alternative: None,
						term: term
					},
					new_pos
				)),
				Some((alternative, newer_pos)) => Some((
					Alternative {
						alternative: Some(Box::new(alternative)),
						term: term
					},
					newer_pos
				))
			}
		}
	}

	fn test(&self, it: &mut Chars) -> bool {
		match &self.alternative {
			None => {},
			Some(val) => {
				if !(self.term.test(it) && val.test(it)) { 
					return false;
				}
			}
		};

		true
	}
}