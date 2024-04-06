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

#[allow(dead_code)]
#[derive(Clone)]
pub enum Node {
  Number(i32),
  String(String),
}
