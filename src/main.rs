mod vm;
mod instruction;
mod field;
mod operation;
mod token;
mod variable;

use crate::vm::VM;


fn main() {

    let file_path = "src/test.txt";
    let mut vm = VM::new();
    vm.execute(file_path);
    vm.print_stack();
}

// TODO
// - [ ] ar fi bun daca as avea si un read ceva