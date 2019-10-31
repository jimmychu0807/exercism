pub mod graph {

  use std::collections::HashMap;

  #[derive(Clone)]
  pub struct Graph {
    pub edges: Vec<graph_items::edge::Edge>,
    pub nodes: Vec<graph_items::node::Node>,
    pub attrs: HashMap<String, String>,
  }

  impl Graph {
    pub fn new() -> Self {
      Self {
        edges: vec![],
        nodes: vec![],
        attrs: HashMap::new()
      }
    }

    pub fn with_nodes(&mut self, nodes: &Vec<graph_items::node::Node>) -> Self {
      self.nodes = (*nodes).clone();
      self.clone()
    }

    pub fn with_edges(&mut self, edges: &Vec<graph_items::edge::Edge>) -> Self {
      self.edges = (*edges).clone();
      self.clone()
    }

    pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Self {
      for (key, val) in attrs {
        self.attrs.insert(key.to_string(), val.to_string());
      }
      self.clone()
    }

  }

  pub mod graph_items {
    pub mod edge {

      use std::collections::HashMap;

      #[derive(Clone, PartialEq, Debug)]
      pub struct Edge {
        pub edge: (String, String),
        pub attrs: HashMap<String, String>,
      }

      impl Edge {
        pub fn new(one: &str, two: &str) -> Self {
          Self {
            edge: (one.to_string(), two.to_string()),
            attrs: HashMap::new(),
          }
        }
      }

    }

    pub mod node {

      use std::collections::HashMap;

      #[derive(Clone, PartialEq, Debug)]
      pub struct Node {
        pub id: String,
        pub attrs: HashMap<String, String>,
      }

      impl Node {
        pub fn new(id: &str) -> Self {
          Self {
            id: id.to_string(),
            attrs: HashMap::new(),
          }
        }

        pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Self {
          for (key, val) in attrs {
            self.attrs.insert(key.to_string(), val.to_string());
          }
          self.clone()
        }
      }

    }
  }
}
