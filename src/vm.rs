use std::collections::VecDeque;
use crate::field::Field;
use crate::instruction::Instruction;
use crate::token::Token;

pub struct VM{}

impl VM{
    pub fn start_vm(file_path: &str) -> Vec<Instruction> {
        let tokens = Token::tokenize_file(file_path);
        // println!("{:?}", tokens);
        let instructions = Instruction::from_tokens(tokens);
        // println!("{:?}", instructions);
        instructions
    }

    pub fn execute(instructions: Vec<Instruction>) -> VecDeque<Field>{
        let mut results: VecDeque<Field> = VecDeque::new();

        for instruction in instructions{
            instruction.execute(&mut results);
        }
        results
    }
}
