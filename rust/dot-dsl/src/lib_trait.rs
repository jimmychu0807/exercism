// Inspired by JPRoland's solution:
//   https://exercism.io/tracks/rust/exercises/dot-dsl/solutions/f40ddaa98c2e4f53a514c90b4ca3e37b
pub mod graph {

  use graph_items::{edge::Edge, node::Node};
  use std::collections::HashMap;

  type Attrs = HashMap<String, String>;

  pub trait Attributes {
    fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self
      where Self: Sized {
      for (key, val) in attrs {
        self.get_attrs().insert(key.to_string(), val.to_string());
      }
      self
    }

    fn get_attr(&self, key: &str) -> Option<&str> {
      self.get_attrs_ro().get(key).map(|v| v.as_str())
    }

    fn get_attrs<'a>(&'a mut self) -> &'a mut Attrs;
    fn get_attrs_ro<'a>(&'a self) -> &'a Attrs;
  }

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
  }

  impl Attributes for Graph {
    fn get_attrs<'a>(&'a mut self) -> &'a mut Attrs { &mut self.attrs }
    fn get_attrs_ro<'a>(&'a self) -> &'a Attrs { &self.attrs }
  }

  pub mod graph_items {
    pub mod edge {

      use super::super::{Attrs, Attributes};

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
      }

      impl Attributes for Edge {
        fn get_attrs<'a>(&'a mut self) -> &'a mut Attrs { &mut self.attrs }
        fn get_attrs_ro<'a>(&'a self) -> &'a Attrs { &self.attrs }
      }

    }

    pub mod node {

      use super::super::{Attrs, Attributes};

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
      }

      impl Attributes for Node {
        fn get_attrs<'a>(&'a mut self) -> &'a mut Attrs { &mut self.attrs }
        fn get_attrs_ro<'a>(&'a self) -> &'a Attrs { &self.attrs }
      }

    }
  }
}
