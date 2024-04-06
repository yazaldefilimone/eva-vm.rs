use crate::bytecode::code;
use crate::node::node;
use crate::utils;
use crate::utils::get_string_value;
use crate::utils::is_number;
use crate::utils::is_string;
use node::Node;
use node::OperationEnum;
use utils::get_number_value;
use utils::STACK_LIMIT;

// use node::NodeNumber;
pub struct VirtualMachine {
  code: Vec<usize>,
  constants: Vec<Node>,
  stack: [Node; STACK_LIMIT],
  instruction_pointer: usize,
  stack_pointer: usize,
}

const ARRAY_REPEAT_VALUE: Node = Node::Number(0);
impl VirtualMachine {
  pub fn new() -> VirtualMachine {
    VirtualMachine {
      code: Vec::new(),
      constants: Vec::new(),
      stack: [ARRAY_REPEAT_VALUE; STACK_LIMIT], // Initialize the stack with zeros
      instruction_pointer: 0,
      stack_pointer: 0,
    }
  }

  pub fn compile(&mut self, _program: String) -> &Node {
    // self.constants.push(Node::Number(10));
    // self.constants.push(Node::Number(3));
    // self.constants.push(Node::Number(5));
    // strings
    self.constants.push(Node::String("Hello, ".to_string()));
    self.constants.push(Node::String("World!".to_string()));
    self.code = [code::CONST, 0, code::CONST, 1, code::ADD, code::HALT].to_vec();
    return self.main_loop();
  }

  pub fn main_loop(&mut self) -> &Node {
    loop {
      let current_byte = self.read_bytes();
      match current_byte {
        code::HALT => {
          return self.pop();
        }
        code::CONST => {
          let constant = self._get_constant().clone();
          self.push(constant);
        }
        code::ADD => {
          let sum = self._binary_operation(OperationEnum::Add);
          self.push(sum);
        }
        code::SUB => {
          let difference = self._binary_operation(OperationEnum::Subtract);
          self.push(difference);
        }
        code::MUL => {
          let product = self._binary_operation(OperationEnum::Multiply);
          self.push(product);
        }
        code::DIV => {
          let quotient = self._binary_operation(OperationEnum::Divide);
          self.push(quotient);
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

  pub fn pop(&mut self) -> &Node {
    if self.stack_pointer == 0 {
      panic!("Empty stack");
    }
    let value = &self.stack[self.stack_pointer];
    self.stack_pointer -= 1;
    value
  }

  // helper functions
  pub fn _get_constant(&mut self) -> &Node {
    let index = self.read_bytes() as i32;
    &self.constants[index as usize]
  }

  pub fn _binary_operation(&mut self, operation: OperationEnum) -> Node {
    // reverse oder of nodes, is last node on stack is first argument...
    let (right, left) = (self.pop().clone(), self.pop().clone());
    let node = match operation {
      OperationEnum::Add => self._add_operation(left, right),
      OperationEnum::Subtract => self._subtract_operation(left, right),
      OperationEnum::Multiply => self._multiply_operation(left, right),
      OperationEnum::Divide => self._divide_operation(left, right),
    };
    node
  }

  fn _subtract_operation(&mut self, left: Node, right: Node) -> Node {
    if is_number(&right) && is_number(&left) {
      return Node::Number(get_number_value(left).unwrap() - get_number_value(right).unwrap());
    }
    panic!("Invalid operation");
  }
  fn _multiply_operation(&mut self, left: Node, right: Node) -> Node {
    if is_number(&right) && is_number(&left) {
      return Node::Number(get_number_value(left).unwrap() * get_number_value(right).unwrap());
    }
    panic!("Invalid operation");
  }

  fn _divide_operation(&mut self, left: Node, right: Node) -> Node {
    if is_number(&right) && is_number(&left) {
      return Node::Number(get_number_value(left).unwrap() / get_number_value(right).unwrap());
    }
    panic!("Invalid operation");
  }

  fn _add_operation(&mut self, left: Node, right: Node) -> Node {
    if is_number(&right) && is_number(&left) {
      return Node::Number(get_number_value(left).unwrap() + get_number_value(right).unwrap());
    }
    if is_string(&right) && is_string(&left) {
      let left_string = get_string_value(&left).unwrap();
      let right_string = get_string_value(&right).unwrap();
      return Node::String(left_string.to_owned() + right_string);
    }
    panic!("Invalid operation");
  }
}
