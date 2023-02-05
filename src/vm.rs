use std::collections::VecDeque;
use crate::field::Field;
use crate::instruction::Instruction;
use crate::token::Token;
use crate::variable::Variable;

pub struct VM{
    pub stack: VecDeque<i64>
}

impl VM{
    pub fn new() -> Self {
        VM{ stack: VecDeque::new() }
    }

    pub fn execute(&mut self, file_path: &str) {
        /// Execute the assembly file
        /// # Arguments
        /// * `file_path` - The path to the assembly file
        let tokens = Token::tokenize_file(file_path);
        // println!("{:?}", tokens); // Debugging

        let processed_tokens = Instruction::from_tokens(tokens);
        // println!("{:?}", processed_tokens); // Debugging

        let instructions = processed_tokens.0;
        // println!("{:?}", instructions); // Debugging

        let variables = processed_tokens.1;
        // println!("{:?}", variables); // Debugging

        let processed_variables = Variable::from_vec_string(variables);
        // println!("{:?}", processed_variables); // Debugging

        let mut results: VecDeque<Field> = VecDeque::new();

        for instruction in instructions{
            instruction.execute_instruction(&mut results, &processed_variables);
        }
        // println!("{:?}", results); // Debugging
        for result in results{
            match result {
                Field::Integer(i) => self.stack.push_back(i),
                Field::String(s) => println!("{}", s)
            }
        }
    }

    pub fn print_stack(&self) {
        println!("{:?}", self.stack);
    }
}
