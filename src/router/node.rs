use super::http::Method;

#[derive(Clone, Copy)]
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
        $node.child_nodes = vec![];
    };
}


#[derive(Debug)]
pub enum NodeError {
  NotSplitted,
}

pub struct Node {
    pub prefix: String,
    pub method: Method,
    pub callback: Option<usize>,
    pub child_nodes: Vec<Node>,
    pub node_kind: NodeKind,
}

impl Node {
    pub fn new(prefix: &str, method: Method, callback: usize, node_kind: NodeKind) -> Node {
      Node { prefix: String::from(prefix), method: method, callback: Some(callback), child_nodes: vec![], node_kind: node_kind }
    }

    pub fn split(&mut self, len: usize) -> Result<&mut Node, NodeError> {
       if len > self.prefix.len() {
         return Err(NodeError::NotSplitted);
       }

        // We create new node with last part of prefix
        let new_node = Node {
            prefix: String::from(&self.prefix[len..]),
            child_nodes: std::mem::replace(&mut self.child_nodes, vec![]),
            callback: self.callback,
            method: self.method,
            node_kind: self.node_kind,
        };

        // The old node will be a middle node
        reset!(self, &self.prefix[..len]);
        self.add_child(new_node);
        Ok(self.child_nodes.last_mut().unwrap())
    }

    pub fn set_cb(&mut self, handler: usize) {
      self.callback = Some(handler);
    }

    pub fn find_child_with_starting_character(& mut self, character: u8) -> Option<&mut Node> {
      for node in self.child_nodes.iter_mut() {
        if node.prefix.as_bytes()[0] == character {
          return Some(node);
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
        self.child_nodes.push(node);
    }
}
