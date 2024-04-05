use crate::bytecode::code;
use crate::node::node::Node;
use crate::utils::STACK_LIMIT;

// use node::NodeNumber;
pub struct VirtualMachine {
  code: Vec<usize>,
  constants: Vec<Node>,
  stack: [Node; STACK_LIMIT],
  instruction_pointer: usize,
  stack_pointer: usize,
}

impl VirtualMachine {
  pub fn new() -> VirtualMachine {
    VirtualMachine {
      code: Vec::new(),
      constants: Vec::new(),
      stack: [Node::Number(0); STACK_LIMIT],
      instruction_pointer: 0,
      stack_pointer: 0,
    }
  }

  pub fn compile(&mut self, _program: String) -> Node {
    self.constants.push(Node::Number(42));
    self.code = [code::OPERATION_CONST, 1, code::OPERATION_HALT].to_vec();
    self.instruction_pointer = self.code[0];

    return self.main_loop();
  }

  pub fn main_loop(&mut self) -> Node {
    loop {
      match self.read_bytes() {
        code::OPERATION_HALT => {
          return self.pop();
        }
        code::OPERATION_CONST => {
          let constant = self.get_constant();
          self.push(Node::Number(constant));
          // break;
        }
        _ => {
          panic!("Unknown operation");
        }
      }
    }
  }

  pub fn read_bytes(&mut self) -> usize {
    let current_byte = self.code[self.instruction_pointer];
    self.instruction_pointer += 1;
    current_byte
  }

  pub fn push(&mut self, value: Node) {
    if self.stack_pointer == STACK_LIMIT {
      panic!("Stack overflow");
    }
    self.stack_pointer += 1;
    self.stack[self.stack_pointer] = value;
  }

  pub fn pop(&mut self) -> Node {
    if self.stack_pointer == 0 {
      panic!("Empty stack");
    }
    let value = self.stack[self.stack_pointer];
    self.stack_pointer -= 1;
    value
  }

  // helper functions
  pub fn get_constant(&mut self) -> i32 {
    let constant = self.read_bytes();
    constant as i32
  }
}
