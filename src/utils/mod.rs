use crate::node::node::Node;

pub const STACK_LIMIT: usize = 512; // Maximum number of items on the stack

pub fn get_number_value(node: Node) -> Option<i32> {
  if let Node::Number(value) = node {
    return Some(value);
  }
  None
}

#[allow(dead_code)]
pub fn get_string_value(node: &Node) -> Option<&String> {
  if let Node::String(value) = node {
    return Some(value);
  }
  None
}
