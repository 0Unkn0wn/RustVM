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
// - [ ] deci sa fie data section asa
// are un numar langa care arata cate elemente o sa fie iniiatilizate
// si efectiv da pop la ele si dupa le baga in stack care o sa fie un vector de field probabil declarat in vm
// cand sunt sunate variabilele alea doar se iau din stack si cam atat
// - [ ] pentru for fac asa copiez instructiunea aia de 3 ori si doar o execut de 3 ori
// - [ ] ar fi bun daca as avea si un read ceva