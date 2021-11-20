use std::{cell::RefMut, collections::HashMap};

use fnv::FnvHashMap;

use super::{debug::logv, http::Method};

#[derive(Clone, Copy, Debug)]
pub enum NodeKind {
    Static,
    Param,
    MatchAll,
    Regex,
    // It's used for a parameter, that is followed by another parameter in the same part
    MultiParam,
}

macro_rules! reset {
    ($node:ident, $prefix:expr) => {
        $node.prefix = String::from($prefix);
        $node.callback = None;
        $node.node_kind = NodeKind::Static;
        $node.child_nodes = FnvHashMap::with_capacity_and_hasher(256, Default::default());
    };
}


#[derive(Debug)]
pub enum NodeError {
  NotSplitted,
}

#[derive(Debug)]
pub struct Node {
    pub prefix: String,
    pub method: Method,
    pub callback: Option<usize>,
    pub child_nodes: FnvHashMap<u8, Node>,
    pub node_kind: NodeKind,
}

impl Node {
    pub fn new(prefix: &str, method: Method, callback: usize, node_kind: NodeKind) -> Node {
      Node { prefix: String::from(prefix), method: method, callback: Some(callback), child_nodes: FnvHashMap::with_capacity_and_hasher(256, Default::default()), node_kind: node_kind }
    }

    pub fn split(&mut self, len: usize) -> Result<&mut Node, NodeError> {
       if len > self.prefix.len() {
         return Err(NodeError::NotSplitted);
       }

        let hash = FnvHashMap::with_capacity_and_hasher(256, Default::default());
        // We create new node with last part of prefix
        let new_node = Node {
            prefix: String::from(&self.prefix[len..]),
            child_nodes: std::mem::replace(&mut self.child_nodes, hash),
            callback: self.callback,
            method: self.method,
            node_kind: self.node_kind,
        };

        // The old node will be a middle node
        reset!(self, &self.prefix[..len]);
        let key = new_node.prefix.as_bytes()[0];
        self.add_child(new_node);
        Ok(self.child_nodes.get_mut(&key).unwrap())
    }

    pub fn set_cb(&mut self, handler: usize) {
      self.callback = Some(handler);
    }

    pub fn child_starting_with_character(& self, character: u8) -> bool {
      self.child_nodes.contains_key(&character)
    }

    pub fn find_child_with_starting_character(& mut self, character: u8) -> Option<&mut Node> {
      self.child_nodes.get_mut(&character)
    }

    pub fn find_child(& self, character: u8) -> Option<&Node> {
      self.child_nodes.get(&character)
    }

    pub fn find_matching_child (&self, path: &str) -> Option<& Node> {
      let node = self.child_nodes.get(&path.as_bytes()[0]);
      if let Some(n) = node {
        if path.len() >= n.prefix.len() {
          return Some(n);
        }
      } 
      None
    }

    pub fn set_kind(&mut self, kind :NodeKind) {
      self.node_kind = kind;
    }

    fn reset(&mut self, prefix: &str) {
        reset!(self, prefix);
    }

    pub fn add_child(&mut self, node: Node) {
        // TODO
        self.child_nodes.insert(node.prefix.as_bytes()[0], node);
    }
}
