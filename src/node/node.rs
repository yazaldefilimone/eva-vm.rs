#[derive(PartialEq)]
#[allow(dead_code)]
enum NodeEnum {
  Number,
}

#[derive(Clone, Copy)]
pub enum Node {
  Number(i32),
}

pub fn get_node_value(node: Node) -> i32 {
  match node {
    Node::Number(n) => n,
  }
}
