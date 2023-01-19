use std::collections::VecDeque;
use crate::field::Field;
use crate::operation::Operation;
use crate::token::Token;

#[derive(Debug)]
pub struct Instruction{
    pub operation: Operation,
    pub operands: VecDeque<Field>
}

impl Instruction {
    pub fn new(operation: Operation, operands: VecDeque<Field>) -> Self {
        Instruction{ operation, operands }
    }

    pub fn from_tokens(tokens: VecDeque<Token>) -> Vec<Self> {
        //println!("{:?}", tokens); // Debugging
        let mut instructions: Vec<Instruction> = Vec::new();
        for token in tokens {
            match token {
                Token::Operation(s) => {
                    let mut operands = VecDeque::new();
                    let mut parts = s.split_whitespace();
                    let operation= Operation::from(parts.next().unwrap());
                    for part in parts {
                        if part.parse::<i64>().is_ok() {
                            operands.push_back(Field::Integer(part.parse::<i64>().unwrap()));
                        } else {
                            operands.push_back(Field::String(part.to_string()));
                        }
                    }
                    instructions.push(Instruction::new(operation, operands));
                },
                Token::Label(s) => {
                    match s.as_str() {
                        "#data" => println!("Data section"),
                        "#code" => println!("Main section"),
                        _ => println!("Label: {}", s)
                    }
                },
                Token::Directive(s) => {
                    match s.as_str() {
                        ".code" => println!("Main program"),
                        _ => println!("Directive: {}", s)
                    }
                },
                _ => {}
            }
        }
        instructions

    }

    pub fn execute(mut self, results: &mut VecDeque<Field>) {
        match self.operation {
            Operation::Add => {
                let a = self.operands.pop_front().unwrap();
                let b = self.operands.pop_front().unwrap();
                match (a, b) {
                    (Field::Integer(a), Field::Integer(b)) => results.push_back(Field::Integer(a + b)),
                    _ => panic!("Invalid operands")
                }
            },
            Operation::Sub => {
                let a = self.operands.pop_front().unwrap();
                let b = self.operands.pop_front().unwrap();
                match (a, b) {
                    (Field::Integer(a), Field::Integer(b)) => results.push_back(Field::Integer(a - b)),
                    _ => panic!("Invalid operands")
                }
            },
            Operation::Mul => {
                let a = self.operands.pop_front().unwrap();
                let b = self.operands.pop_front().unwrap();
                match (a, b) {
                    (Field::Integer(a), Field::Integer(b)) => results.push_back(Field::Integer(a * b)),
                    _ => panic!("Invalid operands")
                }
            },
            Operation::Div => {
                let a = self.operands.pop_front().unwrap();
                let b = self.operands.pop_front().unwrap();
                match (a, b) {
                    (Field::Integer(a), Field::Integer(b)) => results.push_back(Field::Integer(a / b)),
                    _ => panic!("Invalid operands")
                }
            },
            Operation::Print => {
                for operand in self.operands {
                    match operand {
                        Field::Integer(i) => print!("{} ", i),

                        Field::String(s) => print!("{} ", s)
                    }
                }
                println!();
            }
        }
    }

}