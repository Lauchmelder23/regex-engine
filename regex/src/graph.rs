use std::{rc::{Rc, Weak}, cell::RefCell};

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    Character(u32),
    Epsilon,

    NonConsuming
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
            Condition::NonConsuming => {
                return Some(State { 
                    node: self.to.clone(), 
                    pos: state.pos 
                });
            },

            Condition::Epsilon => {
                if state.pos >= data.len() {
                    return None;
                }

                return Some(State {
                    node: self.to.clone(),
                    pos: state.pos + 1
                })
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
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    edges: Vec<Edge>
}

impl Node {
    pub fn new() -> Node {
        Node { edges: vec![] }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
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
    nodes: Vec<GraphNode>,
    start: Option<GraphNode>
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: vec![],
            start: None
        }
    }

    pub fn add_node(&mut self, node: Node) -> GraphNode {
        let graph_node = Rc::new(RefCell::from(node));
        self.nodes.push(graph_node.clone());

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