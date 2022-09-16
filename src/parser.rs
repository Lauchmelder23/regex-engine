use std::{str::FromStr, iter::Peekable, fmt::Display};

use crate::node::{Node, Branch, Quantifier};

#[derive(Debug)]
pub struct ParserError {
	text: String
}

impl ParserError {
	fn new(description: &str) -> ParserError {
		ParserError {
			text: String::from_str(description).ok().unwrap_or_default()
		}
	}
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "ParsingError: {}", self.text)
    }
}

pub type ParserResult = Result<Branch, ParserError>;

#[derive(Debug)]
struct Parser<T> where T: Iterator<Item = u32> + Clone {
	iterator: Peekable<T>
}

impl<T> Parser<T> where T: Iterator<Item = u32> + Clone {
	fn consume_if(&mut self, c: char) -> bool {
		self.iterator.next_if_eq(&c.into()).is_some()
	}

	fn peek_if(&mut self, c: char) -> bool {
		match self.iterator.peek() {
			None => false,
			Some(val) => val == &u32::from(c)
		}
	}

	fn peek_if_any_of(&mut self, set: &str) -> bool {
		match self.iterator.peek() {
			None => false,
			Some(val) => set.contains(char::from_u32(*val).unwrap())
		}
	}

	fn next_if_any_of(&mut self, set: &str) -> Option<T::Item> {
		let val = self.iterator.peek()?;

		if set.contains(char::from_u32(*val).unwrap()) {
			return self.iterator.next();
		}

		None
	}

	fn is_finished(&mut self) -> bool {
		self.iterator.peek().is_none()
	}
	
	fn parse(&mut self) -> ParserResult {
		self.parse_disjunction()
	}

	fn parse_disjunction(&mut self) -> ParserResult {
		let left = self.parse_alternative()?;
		let right = match self.consume_if('|') {
			false => Node::Empty(false).into(),
			true => self.parse_disjunction()?
		};

		Ok(Node::Disjunction(left, right).into())
	}

	fn parse_alternative(&mut self) -> ParserResult {
		let left = self.parse_term()?;
		let right = match !self.peek_if_any_of("|)") && !self.is_finished() {
			false => Node::Empty(true).into(),
			true => { 
				self.parse_alternative()? 
			}
		};
		Ok(Node::Alternative(left, right).into())
	}

	fn parse_term(&mut self) -> ParserResult {
		let tmp = self.iterator.clone()
			.take(2)
			.map(|n| char::from_u32(n).unwrap_or_default())
			.collect::<String>();

		if tmp.starts_with('^') ||
			tmp.starts_with('$') ||
			tmp == "\\b" ||
			tmp == "\\B" ||
			tmp == "(?" 
		{
			todo!()
		}

		let left = self.parse_atom()?;
		let right = match char::from_u32(self.next_if_any_of("*+?{").unwrap_or_default()).unwrap() {
			'*' => Quantifier::NoneOrMore,
			'+' => Quantifier::OneOrMore,
			'?' => Quantifier::OneOrNone,
			'{' => todo!(),
			_ => Quantifier::One
		};

		Ok(Node::Atom(left, right).into())
	}

	fn parse_atom(&mut self) -> ParserResult {
		match self.next_if_any_of(".\\[(") {
			None => self.parse_pattern_char(),

			Some(c) => Ok(
				match char::from_u32(c).unwrap() {
					'.' 	=> Node::AnyCharacter.into(),
					'\\'	=> self.parse_atom_escape()?,
					'(' 	=> { 
						let disjunction = self.parse_disjunction()?;
						self.iterator.next();
						disjunction
					},
					'[' 	=> todo!(),

					_ 		=> unimplemented!()
				}
			)
		}
	}

	fn parse_pattern_char(&mut self) -> ParserResult {
		match self.iterator.next() {
			None => Err(ParserError::new("Pattern ended unexpectedly")),
			Some(c) => {
				if "^$\\.*+?()[]{}|".contains(char::from_u32(c).unwrap_or_default()) {
					return Err(ParserError::new(format!(
						"Unexpected symbol '{}' (U+{}) encountered", 
						char::from_u32(c).unwrap_or_default(), 
						c
					).as_str()));
				}

				Ok(Node::PatternCharacter(c).into())
			}
		}
	}

	fn parse_atom_escape(&mut self) -> ParserResult {
		let c = char::from_u32(self.iterator.next().unwrap_or_default()).unwrap();

		if c.is_numeric() {
			// capoture reference
			todo!()
		} else if "dDsSwW".contains(c) {
			// character class escape
			todo!()
		}

		// handle character escape
		todo!()
	}
}

pub fn parse(regex: &str) -> ParserResult {
	let mut parser = Parser {
		iterator: regex.chars().map(u32::from).peekable()
	};

	parser.parse()
}