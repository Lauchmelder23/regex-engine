use std::rc::Rc;

use crate::{graph::{Graph, Edge, Node, State}, parser::{ParserError, self}};

#[derive(Debug)]
pub struct Regex {
	graph: Graph
}

impl Regex {
	/// Parses a regex pattern into an expression tree
	/// 
	/// # Arguments
	/// 
	/// * `regex` - A string slice that holds a regex pattern
	/// 
	/// # Examples
	/// ```
	/// let regex = Regex::new("Hell.* world");
	/// ```
	pub fn new(pattern: &str) -> Result<Regex, ParserError> {
		let graph = parser::parse(pattern)?;

		Ok(Regex {
			graph: graph
		})
	}

	pub fn test(&self, input: &str) -> bool {
		let safe_input = input.chars().map(u32::from).collect::<Vec<u32>>();
		let mut stack: Vec<State> = vec![State {
			node: self.graph.get_start_node(),
			pos: 0
		}];
		
		while !stack.is_empty() {
			let state = match stack.pop() {
				None => return false,
				Some(item) => item
			};

			if state.node.borrow().is_empty() {
				return true;
			}

			let node = state.node.borrow();
			node.get_edges().for_each(|edge| match edge.try_traverse(&safe_input, &state) {
				None => {},
				Some(new_state) => stack.push(new_state)
			})
		}

		false
	}
}