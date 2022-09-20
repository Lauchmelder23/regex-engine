use std::{str::FromStr, iter::Peekable, fmt::Display, rc::Rc, thread::current};

use crate::graph::{Node, Graph, Condition, Edge, GraphNode};

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

pub type ParserResult = Result<GraphNode, ParserError>;

#[derive(Debug)]
struct Parser<T> where T: Iterator<Item = u32> + Clone {
	iterator: Peekable<T>,
	graph: Graph
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
	fn parse(&mut self) -> Result<Graph, ParserError> {
		let base = self.graph.add_node();
		self.parse_disjunction(&base)?;

		self.graph.set_start_node(&base);

		Ok(self.graph.clone())
	}

	// Parse a disjunction.
	// Disjunctions are disjunctions of alternatives
	fn parse_disjunction(&mut self, base: &GraphNode) -> ParserResult {
		let mut terms: Vec<GraphNode> = vec![];
		let term = self.parse_term(base)?;
		terms.push(term.clone());

		while self.consume_if('|') {
			terms.push(self.parse_term(base)?);
		}

		let end_node = self.graph.add_node();

		{
			terms.into_iter().for_each(|item| {
				item.borrow_mut().push_edge(Edge::new(&end_node, Condition::Epsilon));
			});
		}

		Ok(end_node)
	}

	// Parse term
	// Terms are either assertions, or atoms (with out without quantifiers)
	fn parse_term(&mut self, base: &GraphNode) -> ParserResult {
		let mut current_node = base.clone();

		while !self.peek_if_any_of(ALTERNATIVE_EMPTY) && !self.is_finished() {
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

			let atom = self.parse_atom(&current_node)?;
			let mut next_node = atom.clone();

			{
				let mut cell = atom.borrow_mut();
				match char::from_u32(self.next_if_any_of(QUANTIFIERS).unwrap_or_default()).unwrap() {
					'+' => { 
						cell.push_edge(Edge::new(&current_node, Condition::Epsilon));
					},
					'*' => {
						let exit_node = self.graph.add_node();
						cell.push_edge(Edge::new(&exit_node, Condition::Epsilon));
						cell.push_edge(Edge::new(&current_node, Condition::Epsilon));

						current_node.borrow_mut().add_edge(0, Edge::new(&exit_node, Condition::Epsilon));

						next_node = exit_node.clone();
					},
					'?' => todo!(),
					'{' => todo!(),
					_ => {}
				};
			}

			current_node = next_node;
		};

		Ok(current_node)
	}

	fn parse_atom(&mut self, base: &GraphNode) -> ParserResult {
		if self.consume_if('(') {
			let disjunction = self.parse_disjunction(base)?;
			if !self.consume_if(')') {
				return Err(ParserError::new("Expected ')'"));
			}

			return Ok(disjunction);
		}

		if self.consume_if('.') {
			let new_node = self.graph.add_node();
			base.borrow_mut().add_edge(0, Edge::new(&new_node, Condition::AnyCharacter));
			return Ok(new_node);
		}

		let c = self.parse_pattern_char()?;

		let new_node = self.graph.add_node();
		base.borrow_mut().add_edge(0, Edge::new(&new_node, Condition::Character(c)));
		Ok(new_node)
	}

	// Parse PatternCharacter
	// A PatternCharacter is any unicode char that is not a regex control character
	fn parse_pattern_char(&mut self) -> Result<u32, ParserError> {
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

				Ok(c)
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

pub fn parse(regex: &str) -> Result<Graph, ParserError> {
	let mut parser = Parser {
		iterator: regex.chars().map(u32::from).peekable(),
		graph: Graph::new()
	};

	parser.parse()
}