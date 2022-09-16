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
		let right = match !self.consume_if('|') && !self.is_finished() {
			false => Node::Empty(true).into(),
			true => self.parse_alternative()?
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
		let right = match char::from_u32(*self.iterator.peek().unwrap_or(&0)).unwrap_or_default() {
			'*' => Quantifier::NoneOrMore,
			'+' => Quantifier::OneOrMore,
			'?' => Quantifier::OneOrNone,
			'{' => todo!(),
			_ => Quantifier::One
		};

		Ok(Node::Atom(left, right).into())
	}

	fn parse_atom(&mut self) -> ParserResult {
		Ok(Node::PatternCharacter(self.iterator.next().unwrap()).into())
	}
}

pub fn parse(regex: &str) -> ParserResult {
	let mut parser = Parser {
		iterator: regex.chars().map(u32::from).peekable()
	};

	parser.parse()
}