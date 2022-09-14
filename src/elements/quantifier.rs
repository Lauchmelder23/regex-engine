use std::{str::Chars, iter::Peekable};

use super::element::{Element, ParseResult};

#[derive(Debug)]
pub enum Quantifier {
	ZeroOrMore,
	OneOrMore
}

impl Element for Quantifier {
	fn parse(expr: &str, pos: usize) -> ParseResult<Quantifier> {
		let current_char = match expr.chars().nth(pos) {
			None => return None,
			Some(c) => c
		};

		match current_char {
			'*' => Some((Quantifier::ZeroOrMore, pos + 1)),
			'+' => Some((Quantifier::OneOrMore, pos + 1)),
			_ => None
		}
	}

	fn test(&self, _: &mut Peekable<Chars>) -> bool {
		true
	}
}