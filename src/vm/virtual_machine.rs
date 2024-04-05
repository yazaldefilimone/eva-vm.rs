use crate::bytecode::code;
use crate::node::node;
use crate::utils;
use node::Node;
use utils::get_node_value;
use utils::STACK_LIMIT;

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

  pub fn compile(&mut self, _program: String) -> Option<Node> {
    self.constants.push(Node::Number(2));
    self.constants.push(Node::Number(3));
    self.code = [code::CONST, 0, code::CONST, 1, code::ADD, code::HALT].to_vec();
    return self.main_loop();
  }

  pub fn main_loop(&mut self) -> Option<Node> {
    loop {
      let current_byte = self.read_bytes();
      match current_byte {
        code::HALT => {
          return Some(self.pop());
        }
        code::CONST => {
          let constant = self.get_constant();
          self.push(constant);
        }
        code::ADD => {
          let (left, right) = (self.pop(), self.pop());
          let sum = get_node_value(left) + get_node_value(right);
          self.push(Node::Number(sum));
        }
        code::SUB => {
          let (left, right) = (self.pop(), self.pop());
          let difference = get_node_value(left) - get_node_value(right);
          self.push(Node::Number(difference));
        }
        code::MUL => {
          let (left, right) = (self.pop(), self.pop());
          let product = get_node_value(left) * get_node_value(right);
          self.push(Node::Number(product));
        }
        code::DIV => {
          let (left, right) = (self.pop(), self.pop());
          let quotient = get_node_value(left) / get_node_value(right);
          self.push(Node::Number(quotient));
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
  pub fn get_constant(&mut self) -> Node {
    let index = self.read_bytes() as i32;
    self.constants[index as usize]
  }
}
