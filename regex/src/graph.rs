use std::{rc::{Rc, Weak}, cell::RefCell, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    Character(u32),
    AnyCharacter,
    Epsilon
}

#[derive(Debug)]
pub struct State {
	pub node: GraphNode,
	pub pos: usize
}

#[derive(Debug, Clone)]
pub struct Edge {
    to: GraphNode,
    condition: Condition
}

impl Edge {
    pub fn new(to: &GraphNode, condition: Condition) -> Edge {
        Edge {
            to: to.clone(),
            condition: condition
        }
    }

    pub fn try_traverse(&self, data: &Vec<u32>, state: &State) -> Option<State> {
        match self.condition {
            Condition::Epsilon => {
                return Some(State { 
                    node: self.to.clone(), 
                    pos: state.pos 
                });
            },

            Condition::Character(ch) => {
                if state.pos >= data.len() {
                    return None;
                }

                if ch == data[state.pos] {
                    return Some(State {
                        node: self.to.clone(),
                        pos: state.pos + 1
                    })
                }
            },

            Condition::AnyCharacter => {
                if state.pos < data.len() {
                    return Some(State {
                        node: self.to.clone(),
                        pos: state.pos + 1
                    });
                }
            }

        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    id: usize,
    edges: Vec<Edge>
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Node {
    pub fn new(id: usize) -> Node {
        Node { edges: vec![], id: id }
    }

    pub fn push_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn add_edge(&mut self, position: usize, edge: Edge) {
        self.edges.insert(position, edge);
    }

    pub fn get_edges(&self) -> std::slice::Iter<'_, Edge>{
        self.edges.iter()
    }
    
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

pub type GraphNode = Rc<RefCell<Node>>;

#[derive(Debug, Clone)]
pub struct Graph {
    next_id: usize,
    nodes: Vec<GraphNode>,
    start: Option<GraphNode>
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            next_id: 0,
            nodes: vec![],
            start: None
        }
    }

    pub fn add_node(&mut self) -> GraphNode {
        let graph_node = Rc::new(RefCell::from(Node::new(self.next_id)));
        self.nodes.push(graph_node.clone());
        self.next_id += 1;

        graph_node
    }

    pub fn set_start_node(&mut self, node: &GraphNode) {
        self.start =Some(node.clone());
    }

    pub fn get_start_node(&self) -> GraphNode {
        // At this point the start node should be set
        self.start.as_ref().unwrap().clone()
    }
}