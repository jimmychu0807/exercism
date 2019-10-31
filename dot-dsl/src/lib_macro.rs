// Inspired by: https://exercism.io/tracks/rust/exercises/dot-dsl/solutions/2fb7f362a5d54bb1a9bccb301e2c4965

macro_rules! with_attrs {
  () => {
    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
      for (key, value) in attrs {
        self.attrs.insert(key.to_string(), value.to_string());
      }
      self
    }

    pub fn get_attr(&self, name: &str) -> Option<&str> {
      self.attrs.get(name).map(|v| v.as_str())
    }
  }
}

pub mod graph {

  use graph_items::{edge::Edge, node::Node};
  use std::collections::HashMap;

  type Attrs = HashMap<String, String>;

  #[derive(Default, Clone)]
  pub struct Graph {
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
    pub attrs: Attrs,
  }

  impl Graph {
    pub fn new() -> Self {
      Self::default()
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
      self.nodes = nodes.into();
      self
    }

    pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
      self.edges = (*edges).clone();
      self
    }

    pub fn get_node(&self, node_id: &str) -> Option<&Node> {
      self.nodes.iter().find(|node| node.id == node_id)
    }

    with_attrs!();
  }

  pub mod graph_items {
    pub mod edge {

      use super::super::Attrs;

      #[derive(Clone, PartialEq, Debug)]
      pub struct Edge {
        pub edge: (String, String),
        pub attrs: Attrs,
      }

      impl Edge {
        pub fn new(one: &str, two: &str) -> Self {
          Self {
            edge: (one.to_string(), two.to_string()),
            attrs: Attrs::new(),
          }
        }
        with_attrs!();
      }
    }

    pub mod node {

      use super::super::Attrs;

      #[derive(Clone, PartialEq, Debug, Default)]
      pub struct Node {
        pub id: String,
        pub attrs: Attrs,
      }

      impl Node {
        pub fn new(id: &str) -> Self {
          Self {
            id: id.to_string(),
            attrs: Attrs::new(),
          }
        }
        with_attrs!();
      }
    }
  }
}
