use TSPL::Parser;

use self::ast::Node;
TSPL::new_parser!(EvaParser);
mod ast;

impl<'a> EvaParser<'a> {
  pub fn parse(&mut self) -> Node {
    self.skip_trivia();
    match self.peek_one() {
      Some('0'..='9') => self.parse_number(),
      Some('(') => self.parse_expression(),
      // strings
      Some('"') => self.parse_string(),
      // math
      Some('+') => self.parse_math(),
      Some('-') => self.parse_math(),
      Some('*') => self.parse_math(),
      Some('/') => self.parse_math(),
      //  keywords, name, var, let, fn, if, else, while, return
      Some('a'..='z') => self.parse_keyword(),
      _ => panic!("Unexpected token: {:?}", self.peek_one()),
    }
  }

  // (), (+ 1 2), (+ 1 (+ 2 3) 4), (push an, 10)
  pub fn parse_expression(&mut self) -> Node {
    self.skip_trivia();
    self.advance_one(); // consume the opening parenthesis
    self.skip_trivia();
    let mut nodes: Vec<Node> = Vec::new();
    while let Some(')') = self.peek_one() {
      nodes.push(self.parse());
      self.skip_trivia();
    }
    self.advance_one(); // consume the closing parenthesis
    Node::List(nodes)
  }
  pub fn parse_number(&mut self) -> Node {
    let mut number = String::new();
    self.skip_trivia();
    while let Some('0'..='9') = self.peek_one() {
      self.skip_trivia();
      number.push(self.advance_one().unwrap());
    }
    self.skip_trivia();
    Node::Number(number.parse().unwrap())
  }

  pub fn parse_string(&mut self) -> Node {
    self.skip_trivia();
    self.advance_one(); // consume the opening quote
    let mut string = String::new();
    self.skip_trivia();
    while let Some(c) = self.advance_one() {
      if c == '"' {
        break;
      }
      string.push(c);
    }
    Node::String(string)
  }

  pub fn parse_math(&mut self) -> Node {
    self.skip_trivia();
    let operator = self.advance_one().unwrap();
    self.skip_trivia();
    let left = self.parse();
    self.skip_trivia();
    let right = self.parse();
    self.skip_trivia();
    Node::List(vec![Node::Symbol(operator.to_string()), left, right])
  }

  pub fn parse_keyword(&mut self) -> Node {
    self.skip_trivia();
    let keyword = self.parse_name().unwrap();
    self.skip_trivia();
    Node::Symbol(keyword)
  }
}
