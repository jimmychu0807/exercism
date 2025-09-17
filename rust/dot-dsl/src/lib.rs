pub mod graph_items;

pub mod graph {
	pub use crate::graph_items;
	pub use graph_items::{edge::Edge, node::Node};
	use std::collections::HashMap;

	#[derive(Debug, Default, Clone)]
	pub struct Attr {}

	#[derive(Debug, Default, Clone)]
	pub struct Graph {
		pub edges: Vec<Edge>,
		pub nodes: Vec<Node>,
		pub attrs: HashMap<String, String>,
	}

	impl Graph {
		pub fn new() -> Self {
			Self::default()
		}

		pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
			self.nodes = nodes.to_vec();
			self
		}

		pub fn with_edges(mut self, edges: &[Edge]) -> Self {
			self.edges = edges.to_vec();
			self
		}

		pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
			for (key, val) in attrs {
				self.attrs.insert(key.to_string(), val.to_string());
			}

			self
		}

		pub fn node(&self, name: &str) -> Option<&Node> {
			self.nodes.iter().find(|node| name == node.name)
		}
	}
}
