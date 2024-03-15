use crate::bytecode::code;
use crate::value::EvaValue;
use crate::DIE;
use std::mem;

pub struct EvaVM<'a> {
    code: Vec<usize>,
    constants: Vec<EvaValue>,
    stack: [EvaValue; 512],
    ip: usize,                // instruction pointer (aka program counter)
    sp: Option<&'a EvaValue>, // stack ponter
}

impl EvaVM<'static> {
    pub fn new() -> EvaVM<'static> {
        EvaVM {
            code: vec![],
            constants: vec![],
            stack: unsafe { mem::zeroed() },
            ip: 0,
            sp: None,
        }
    }

    fn push(&mut self, value: &'static EvaValue) {
        match self.sp {
            Some(pointer) => {
                if pointer - &self.stack[0] == 512 {
                    DIE!("Stack limit")
                }
            }
            _ => (),
        };

        self.sp += 1
    }

    pub fn exec(&mut self, program: String) {
        let _ = program;
        self.constants.push(EvaValue::new_number(42));
        self.code = vec![code::OPERATION_CONST, 0, code::OPERATION_HALT];
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
                code::OPERATION_CONST => {
                    let index = self.read_byte();
                    let constValue = &self.constants[index];
                    // push(constValue)
                }
                _ => {
                    // DIE!("Unknown opcode: {}", opcode);
                    DIE!("Unknown opcode: {}", opcode);
                }
            }
        }
    }

    fn read_byte(&mut self) -> usize {
        let byte = self.code[self.ip] as usize;
        self.ip += 1;
        byte
    }
}
