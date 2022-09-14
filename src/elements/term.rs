use std::str::Chars;

use super::{atom::Atom, element::{Element, ParseResult}};

#[derive(Debug)]
enum TermType {
	Atom(Atom)
}

#[derive(Debug)]
pub struct Term {
	term_type: TermType
}

impl Element for Term {
	fn parse(expr: &str, pos: usize) -> ParseResult<Term> {
		match <Atom as Element>::parse(expr, pos) {
			None => {},
			Some((atom, new_pos)) => { return Some((
				Term { term_type: TermType::Atom(atom) }, 
				new_pos
			)); }
		}

		None
	}

	fn test(&self, it: &mut Chars) -> bool {
		match &self.term_type {
			TermType::Atom(atom) => atom.test(it)
		}
	}
}