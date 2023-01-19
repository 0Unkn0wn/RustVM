mod vm;
mod instruction;
mod field;
mod operation;
mod token;

use crate::vm::VM;


fn main() {

    let file_path = "src/test.txt";
    let instructions = VM::start_vm(file_path);
    VM::execute(instructions);
}