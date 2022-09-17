use std::{str::FromStr, iter::Peekable, fmt::Display};

use crate::node::{Node, Branch, Quantifier};

static ALTERNATIVE_EMPTY: &'static [u32] = &[
	b'|' as u32, 
	b')' as u32
];

static QUANTIFIERS: &'static [u32] = &[
	b'*' as u32, 
	b'+' as u32,
	b'?' as u32,
	b'{' as u32,
];

static ATOM_SPECIAL_CHARS: &'static [u32] = &[
	b'.' as u32, 
	b'\\' as u32,
	b'[' as u32,
	b'(' as u32,
];

static NOT_PATTERN_CHARS: &'static [u32] = &[
	b'^' as u32, 
	b'$' as u32,
	b'\\' as u32,
	b'.' as u32,
	b'*' as u32,
	b'+' as u32,
	b'?' as u32,
	b'(' as u32,
	b')' as u32,
	b'[' as u32,
	b']' as u32,
	b'{' as u32,
	b'}' as u32,
	b'|' as u32,
];

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
	// Checks whether the next character is equal to c. If yes, it is consumed
	fn consume_if(&mut self, c: char) -> bool {
		self.iterator.next_if_eq(&c.into()).is_some()
	}

	// Check if the next character is equal to c
	fn peek_if(&mut self, c: char) -> bool {
		match self.iterator.peek() {
			None => false,
			Some(val) => val == &u32::from(c)
		}
	}

	// Check if the nect character is equal to any char in the set
	fn peek_if_any_of(&mut self, set: &[u32]) -> bool {
		match self.iterator.peek() {
			None => false,
			Some(val) => set.contains(val)
		}
	}

	// Advance iterator if character is equal to any char in the set
	fn next_if_any_of(&mut self, set: &[u32]) -> Option<T::Item> {
		let val = self.iterator.peek()?;

		if set.contains(val) {
			return self.iterator.next();
		}

		None
	}

	// Check if iterator has reached the end of the string
	fn is_finished(&mut self) -> bool {
		self.iterator.peek().is_none()
	}
	
	// Parse the string represented by the iterator
	fn parse(&mut self) -> ParserResult {
		self.parse_disjunction()
	}

	// Parse a disjunction.
	// Disjunctions are disjunctions of alternatives
	fn parse_disjunction(&mut self) -> ParserResult {
		let left = self.parse_alternative()?;
		let right = match self.consume_if('|') {
			false => Node::Empty(false).into(),
			true => self.parse_disjunction()?
		};

		Ok(Node::Disjunction(left, right).into())
	}

	// Parse alternative
	// Alternatives are sequences of terms
	fn parse_alternative(&mut self) -> ParserResult {
		let mut terms = vec![];
		while !self.peek_if_any_of(ALTERNATIVE_EMPTY) && !self.is_finished() {
			// let term = ;
			terms.push(self.parse_term()?);
		};
		
		let mut alternative = terms.iter().rfold(Node::Empty(true).into(), |acc, term| {
			match &**term {
				Node::Term(atom, quantifier, _) => Node::Term(atom.clone(), *quantifier, acc).into(),
				_ => unimplemented!()
			}	
		});

		alternative = dbg!(alternative);
		Ok(alternative)
	}

	// Parse term
	// Terms are either assertions, or atoms (with out without quantifiers)
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

		let atom = self.parse_atom()?;
		let quantifier = match char::from_u32(self.next_if_any_of(QUANTIFIERS).unwrap_or_default()).unwrap() {
			'*' => Quantifier::new(0, usize::MAX, true),
			'+' => Quantifier::new(1, usize::MAX, true),
			'?' => Quantifier::new(0, 1, true),
			'{' => todo!(),
			_ => Quantifier::new(1, 1, true)
		};

		Ok(Node::Term(atom, quantifier, Node::Empty(true).into()).into())
	}

	// Parse atom
	// Atoms are characters, classes, capture groups etc
	fn parse_atom(&mut self) -> ParserResult {
		match self.next_if_any_of(ATOM_SPECIAL_CHARS) {
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

	// Parse PatternCharacter
	// A PatternCharacter is any unicode char that is not a regex control character
	fn parse_pattern_char(&mut self) -> ParserResult {
		match self.iterator.next() {
			None => Err(ParserError::new("Pattern ended unexpectedly")),
			Some(c) => {
				if NOT_PATTERN_CHARS.contains(&c) {
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

	// Parse AtomEscape
	// AtomEscapes are either normal escapes, or capturing references, or unicode escapes
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

/// Parses a regex pattern into an expression tree
/// 
/// # Arguments
/// 
/// * `regex` - A string slice that holds a regex pattern
/// 
/// # Examples
/// ```
/// let regex = parse("Hell.* world");
/// ```
pub fn parse(regex: &str) -> ParserResult {
	let mut parser = Parser {
		iterator: regex.chars().map(u32::from).peekable()
	};

	parser.parse()
}