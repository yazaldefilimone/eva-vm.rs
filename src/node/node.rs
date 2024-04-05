#[derive(PartialEq)]
#[allow(dead_code)]
enum NodeEnum {
  Number,
}

#[derive(Clone, Copy)]
pub enum Node {
  Number(i32),
}
