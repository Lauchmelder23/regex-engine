use std::str::Chars;

use super::element::{Element, ParseResult};

#[derive(Debug)]
enum AtomType {
	PatternCharacter(char),
	Wildcard
}

#[derive(Debug)]
pub struct Atom {
	atom_type: AtomType
}

fn is_source_char(c: char) -> bool {
	!"^$\\.*+?()[]{}|".contains(c)
}

impl Element for Atom {
	fn parse(expr: &str, pos: usize) -> ParseResult<Atom> {
		let current_char = match expr.chars().nth(pos) {
			None => return None,
			Some(c) => c
		};

		match current_char {
			'.' => Some((
				Atom { atom_type: AtomType::Wildcard }, 
				pos + 1
			)),
			_ => {
				if is_source_char(current_char) {
					Some((
						Atom { atom_type: AtomType::PatternCharacter(current_char) }, 
						pos + 1
					))
				} else {
					None
				}
			}
		}
	}

	fn test(&self, it: &mut Chars) -> bool {
		let next_char = match it.next() {
			None => return false,
			Some(c) => c
		};
		
		match self.atom_type  {
			AtomType::PatternCharacter(c) => (next_char == c),
			AtomType::Wildcard => true
		}
	}
}