use std::{str::Chars, iter::Peekable};

use super::{atom::Atom, element::{Element, ParseResult}, quantifier::Quantifier};

#[derive(Debug)]
enum TermType {
	Atom(Atom)
}

#[derive(Debug)]
pub struct Term {
	term_type: TermType,
	quantifier: Option<Quantifier>
}

impl Term {
	fn test_atom(&self, atom: &Atom, it: &mut Peekable<Chars>) -> bool {
		match self.quantifier {
			None => atom.test(it),
			Some(Quantifier::ZeroOrMore) => {
				let mut next_char = match it.peek() {
					None => { return true; }
					Some(c) => c
				};
				
				while atom.test_char(*next_char) {
					it.next();
					next_char = match it.peek() {
						None => { return true; }
						Some(c) => c
					};
				}

				true
			}
			Some(Quantifier::OneOrMore) => {
				if !atom.test(it) {
					return false;
				}

				let mut next_char = match it.peek() {
					None => { return true; }
					Some(c) => c
				};
				
				while atom.test_char(*next_char) {
					it.next();
					next_char = match it.peek() {
						None => { return true; }
						Some(c) => c
					};
				}
				
				true
			}
		}
	}
}

impl Element for Term {
	fn parse(expr: &str, pos: usize) -> ParseResult<Term> {
		match <Atom as Element>::parse(expr, pos) {
			None => {},
			Some((atom, new_pos)) => { 
				let (quantifier, final_pos) = match <Quantifier as Element>::parse(expr, new_pos) {
					None => (None, new_pos),
					Some((quantifier, post_quantifier_pos)) => (Some(quantifier), post_quantifier_pos)
				};

				return Some((
					Term { 
						term_type: TermType::Atom(atom),
						quantifier: quantifier
					}, 
					final_pos
			)); }
		}

		None
	}

	fn test(&self, it: &mut Peekable<Chars>) -> bool {
		match &self.term_type {
			TermType::Atom(atom) => self.test_atom(atom, it)
		}
	}
}