// Definimos um enum para representar os diferentes tipos de valor
#[derive(PartialEq)]
enum EvaValueType {
  Number,
  // Você pode adicionar mais tipos aqui, se necessário
}

// Definimos uma struct para armazenar o valor e seu tipo
#[derive(PartialEq)]
pub struct EvaValue {
  value_type: EvaValueType,
  number: Option<i32>, // Usamos Option para representar um valor que pode estar presente ou não
}

// Implementamos um construtor para criar um EvaValue a partir de um número
impl EvaValue {
  pub fn new_number(value: i32) -> Self {
      EvaValue {
          value_type: EvaValueType::Number,
          number: Some(value),
      }
  }
}