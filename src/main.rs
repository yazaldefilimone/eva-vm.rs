mod bytecode;
mod node;
mod utils;
mod vm;
use node::node::as_number;
use node::node::Node;
use std::fmt::Debug;
use vm::virtual_machine::VirtualMachine;

impl Debug for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Node::Number(n) => write!(f, "Number({})", n),
    }
  }
}

fn main() {
  let program = "42".to_string();
  let mut vm = VirtualMachine::new();
  let result = vm.compile(program);
  println!("{:?}", as_number(result.unwrap()));
  println!("All done!");
}
