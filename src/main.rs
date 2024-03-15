mod bytecode;
mod logger;
mod vm;
mod value;


fn main() {
    let program = "".to_string();
    vm::eva_vm::EvaVM::new().exec(program);
    println!("All done!");
}
