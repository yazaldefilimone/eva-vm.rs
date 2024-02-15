mod bytecode;

struct EvaVirtualMachine {
  ip: usize,
  code: Vec<u8>,
}


pub impl EvaVirtualMachine {
  pub fn new() -> EvaVirtualMachine {
    EvaVirtualMachine {
      ip: 0,
      code: Vec::new(),
    }
  }


  pub fn exec(&mut self, program: String) -> eval::Result<()> {
    self.ip = &code[0];
    return self.eval();

  }

  pub fn eval(&mut self) {
    loop {
      match self.code[self.ip] {
        bytecode::code::OPERATION_HALT => {
          return;
        }
      }
    }
  }

}

