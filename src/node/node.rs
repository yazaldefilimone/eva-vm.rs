#[derive(PartialEq)]
#[allow(dead_code)]
enum NodeEnum {
  Number,
}
pub enum OperationEnum {
  Add,
  Subtract,
  Multiply,
  Divide,
}
#[derive(Clone, Copy)]
pub enum Node {
  Number(i32),
}
