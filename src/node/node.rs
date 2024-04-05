#[derive(PartialEq)]
enum NodeEnum {
  Number,
}

#[derive(Clone, Copy)]
pub enum Node {
  Number(i32),
}
