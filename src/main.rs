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

// TODO
// - [ ] deci sa fie data section asa
// are un numar langa care arata cate elemente o sa fie iniiatilizate
// si efectiv da pop la ele si dupa le baga in stack care o sa fie un vector de field probabil declarat in vm
// cand sunt sunate variabilele alea doar se iau din stack si cam atat
// - [ ] pentru for fac asa copiez instructiunea aia de 3 ori si doar o execut de 3 ori
// - [ ] ar fi bun daca as avea si un read ceva