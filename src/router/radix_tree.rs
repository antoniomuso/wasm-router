
use super::debug::logv;
use super::http::Method;
use super::node::{Node, NodeKind};
use regex::Regex;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

const OPTIONAL_PARAM_REGEXP: &str = r"(/:[^/()]*?)\?(/?)";

#[wasm_bindgen]
#[derive(Debug)]
pub struct Router {
    default_route: Option<String>,
    on_bad_url: Option<String>,
    ignore_trailing_slash: bool,
    // To refactor from hashmap to array
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

    pub fn lookup(&self, method: Method, route: &str) -> Result<usize, JsValue> {
        // To check the existence 
        let mut current_node = self.trees.get(&method).unwrap();

        let mut path = route;
        loop {

            let mut path_len = path.len();
            let prefix = &current_node.prefix;

            if path_len == 0 || route == prefix {
                if let Some(cb) = current_node.callback {
                    return Ok(cb);
                }
                return Err(JsValue::from_str("Route does not exist"));
            }


            let prefix_len = prefix.len();
            let mut len = 0;

            len = long_common_prefix(path, prefix);

            if len == prefix_len {
                path = &path[len..];
            } else {
                return Err(JsValue::from("Route does not exist"));
            }

            let node = current_node.find_matching_child(path);

            if let Some (n) = node {
                current_node = n;
            }

            if len == 0 {
                return Err(JsValue::from("Route does not exist"));
            }

        }
        // TO implement
    }

    pub fn insert(&mut self, method: Method, path: &str, func: usize) -> JsValue {
        assert!(!path.is_empty(), "route is empty");

        let root = self.trees.get_mut(&method);

        let mut curr_node = match root {
            None => {
                let node = Node::new(path, method,func, NodeKind::Static);
                self.trees.insert(method, node);
                self.trees.get_mut(&method).unwrap()
            }
            Some(node) => node,
        };

        let mut path = path;
        let mut len;
        loop {
            let prefix = &curr_node.prefix;
            let curr_prefix_len = prefix.len();
            let path_len = path.len();

            // max shared len
            len = long_common_prefix(prefix, path);

            // the longest common prefix is smaller than the current prefix
            // let's split the node and add a new child
            if len < curr_prefix_len {
                // path_len <= curr_prefix_len
                curr_node.split(len).unwrap();

                if len == path_len {
                    curr_node.set_cb(func);
                    // For now only static route
                    curr_node.set_kind(NodeKind::Static);
                } else {
                    // len is smaller than path_len
                    let node = Node::new(&path[len..], method,func, NodeKind::Static);
                    curr_node.add_child(node);
                }
            } else if len < path_len {
                path = &path[len..];

                let char = path.bytes().next().unwrap();
                // At the moment we iterate all label, we don't use hashmap
                if !curr_node.child_starting_with_character(char) {
                    let node = Node::new(path, method,func, NodeKind::Static);
                    curr_node.add_child(node);
                } else {
                    curr_node = curr_node.find_child_with_starting_character(char).unwrap();
                    continue;
                }
                
            } else {
                // here the node exist, at the moment we overwrite the handler in next implementation we need to use an handler array
                curr_node.set_cb(func);
            }
            break;
        }

        JsValue::from_str(&format!("{:?}", &self))
    }

    pub fn on(&mut self, method: Method, path: &str, handler: usize) -> bool {
        if path.is_empty() {
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

        self.iter_on(method, path, handler);

        if self.ignore_trailing_slash && path != "/" && !path.ends_with('*') {
            if path.ends_with('/') {
                return self.iter_on(method, &path[0..path.len() - 1], handler);
            } else {
                let str_val = format!("{}/", path);
                return self.iter_on(method, &str_val, handler);
            }
        }

        false
    }

    #[wasm_bindgen]
    pub fn iter_on(&mut self, method: Method, path: &str, handler: usize) -> bool {
        // Todo Checks
        let mut curr_path: String = String::from(path);
        let mut len = curr_path.len();

        let mut bytes: Vec<char> = path.chars().collect();

        // Replace double colon with a single colon
        curr_path = curr_path.replace("::", ":");

        let j = 0;
        let i = 0;

        while i < curr_path.len() {}

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
