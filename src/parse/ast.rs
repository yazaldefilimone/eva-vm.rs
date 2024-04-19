pub enum Node {
  Number(i32),
  String(String),
  List(Vec<Node>),
  Symbol(String),
  EOF,
}
