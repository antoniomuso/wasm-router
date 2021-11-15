use std::{collections::HashMap, error::Error, fmt::Display};
use wasm_bindgen::prelude::*;
use regex::{Regex, bytes};

use super::http::Method;

const OPTIONAL_PARAM_REGEXP: &str = r"(/:[^/()]*?)\?(/?)";

#[derive(Default, Debug)]
struct RouterError {
  msg: &'static str,
}

impl Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl RouterError {
  pub fn new (msg: &'static str) -> Self {
    RouterError {
      msg: msg,
    }
  }
}

#[wasm_bindgen]
pub struct Node {
  prefix: String,
  method: Method,
  callback: Option<String>,
  child_nodes: Vec<Node>,
}

#[wasm_bindgen]
impl Node {
  fn split(&mut self, len: usize) -> Result<Node, RouterError> {
    assert!(len <= self.prefix.len(), "");
    let new_pref: &str = &self.prefix[0..len];
    let new_node = Node {
      prefix: String::from(new_pref),
      child_nodes: std::mem::replace(&mut self.child_nodes, vec![]),
      callback: Option::None,
      method: self.method,
    };

    Ok(new_node)
  }
}

#[wasm_bindgen]
pub struct Router {
  default_route: Option<String>,
  on_bad_url: Option<String>,
  ignore_trailing_slash: bool,
  trees: HashMap<Method, Node>,
  optional_regex: Regex,
}


impl Default for Router {
  fn default() -> Self {
    Self {
      default_route: Default::default(),
      on_bad_url: Default::default(),
      trees: Default::default(),
      ignore_trailing_slash: Default::default(),
      optional_regex: Regex::new(OPTIONAL_PARAM_REGEXP).unwrap(),
    }
  }
}

#[wasm_bindgen]
impl Router {
  pub fn new() -> Self {
    Router {
      ..Default::default()
    }
  }

  pub fn set_default_route(&mut self, default_route: &str) {
    self.default_route.replace(String::from(default_route));
  }

  pub fn set_on_bad_url(&mut self, on_bad_url: &str) {
    self.on_bad_url.replace(String::from(on_bad_url));
  }

  pub fn lookup(&self, method: Method, route: &str) {
    // TO implement
  }

  pub fn add(&mut self, method: Method, path: &str, func: &str) -> u32 {
    assert!(!path.is_empty(), "route is empty");

    let root = self.trees.get(&method);

    if let None = root {
      let node = Node {
        callback: Option::Some(String::from(func)),
        method: method,
        child_nodes: vec![],
        prefix: path.to_string(),
      };
      self.trees.insert(method, node);
      return 0;
    }

    // Cannot be None
    let currNode = root.unwrap();
    let mut len = 0;

    loop {
      let prefix = &currNode.prefix;
      let prefix_len = prefix.len();
      let path_len = path.len();

      // max shared len
      len = long_common_prefix(prefix, path);

      // the longest common prefix is smaller than the current prefix
      // let's split the node and add a new child
      if len < prefix_len {
      } else if len < path_len {
      } else {
      }
    }

    0
  }

  pub fn on (&mut self, method: Method, path: &str, handler: i32) -> bool {
    if path.is_empty(){
      return false;
    }

    let capture = self.optional_regex.captures(path);
    if let Some(caps) = capture {
      let mat = caps.get(0).unwrap();
      assert!(path.len() == mat.end(), "Match is not at the end");

      let full_path = self.optional_regex.replace(path, "$1$2");
      let optional_path = self.optional_regex.replace(path, "$1");

      println!("{} {}", full_path, optional_path);

      self.on(method, &full_path, handler);
      self.on(method, &optional_path, handler);

      return true; 
    }

    if self.ignore_trailing_slash && path != "/" && !path.ends_with('*') {
      if path.ends_with('/') {
        return self.iter_on(method, &path[0..path.len()-1], handler)
      } else {
        let str_val = format!("{}/", path);
        return self.iter_on(method, &str_val, handler)
      }
    }

    false
  }

  #[wasm_bindgen]
  pub fn iter_on(&mut self, method: Method, path: &str, handler: i32) -> bool {
    let bytes = path.as_bytes();
    // Todo Checks 

    for i in 0..bytes.len() {
      if bytes[i] == 58 {
        if i != path.len() - 1 && bytes[i + 1] == 58 {
          
        }
      }
    }
    for (i, asci_char) in bytes.into_iter().enumerate() {
      
     
      
    }


    true
  }
}

#[inline]
fn long_common_prefix(str1: &str, str2: &str) -> usize {
  let arr1 = str1.as_bytes();
  let arr2 = str2.as_bytes();

  let min = if arr1.len() < arr2.len() {
    arr1.len()
  } else {
    arr2.len()
  };

  for i in 0..min {
    if arr1[i] != arr2[i] {
      return i;
    }
  }

  min
}
