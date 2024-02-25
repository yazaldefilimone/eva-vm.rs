use crate::bytecode::code;
use crate::DIE;
pub struct EvaVM {
    code: Vec<u8>,
    ip: usize, // instruction pointer (aka program counter)
}
impl EvaVM {
    pub fn new() -> EvaVM {
        EvaVM {
            code: Vec::new(),
            ip: 0,
        }
    }

    pub fn exec(&mut self, program: String) {
        let _ = program;
        self.code = vec![code::OPERATION_HALT];
        self.eval()
    }

    // main loop
    fn eval(&mut self) -> () {
        loop {
            let opcode = self.read_byte();
            match opcode {
                code::OPERATION_HALT => {
                    println!("HALT");
                    return;
                }
                _ => {
                    // DIE!("Unknown opcode: {}", opcode);
                    DIE!("Unknown opcode: {}", opcode);
                    return;
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte: u8 = self.code[self.ip];
        self.ip += 1;
        byte
    }
}
