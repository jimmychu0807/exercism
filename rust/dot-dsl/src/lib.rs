pub mod graph_items;

pub mod graph {
	pub use crate::graph_items;
	pub use graph_items::{edge::Edge, node::Node};

	#[derive(Debug, Default, Clone)]
	pub struct Attr {}

	#[derive(Debug, Default, Clone)]
	pub struct Graph {
		pub edges: Vec<Edge>,
		pub nodes: Vec<Node>,
		pub attrs: Vec<Attr>,
	}

	impl Graph {
		pub fn new() -> Self {
			Self::default()
		}

		// pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
		// 	self.nodes = nodes.into();
		// 	self
		// }

		// pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
		// 	self.edges = (*edges).clone();
		// 	self
		// }

		// pub fn get_node(&self, node_id: &str) -> Option<&Node> {
		// 	self.nodes.iter().find(|node| node.id == node_id)
		// }
	}
}
