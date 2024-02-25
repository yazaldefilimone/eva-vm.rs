mod bytecode;
mod logger;
mod vm;

fn main() {
    let program = "".to_string();
    vm::eva_vm::EvaVM::new().exec(program);
}
