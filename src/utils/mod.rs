use crate::node::node::Node;

pub const STACK_LIMIT: usize = 512; // Maximum number of items on the stack

pub fn get_node_value(node: Node) -> i32 {
  match node {
    Node::Number(n) => n,
  }
}
