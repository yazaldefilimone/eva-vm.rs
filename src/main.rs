mod bytecode;
mod node;
mod parse;
mod utils;
mod vm;
use node::node::Node;
use std::fmt::Debug;
// use utils::get_number_value;
use parse;
use vm::virtual_machine::VirtualMachine;
impl Debug for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Node::Number(n) => write!(f, "Number({})", n),
      Node::String(s) => write!(f, "String({})", s),
    }
  }
}

fn main() {
  let program = "42".to_string();
  let mut vm = VirtualMachine::new();
  let result = vm.compile(program);
  println!("{:?}", result);
  println!("All done!");
}
