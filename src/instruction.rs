use std::collections::VecDeque;
use crate::field::Field;
use crate::operation::Operation;
use crate::token::Token;
use crate::variable::Variable;

#[derive(Debug)]
pub struct Instruction{
    pub operation: Operation,
    pub operands: VecDeque<Field>
}

impl Instruction {
    pub fn new(operation: Operation, operands: VecDeque<Field>) -> Self {
        Instruction{ operation, operands }
    }

    pub fn from_tokens(mut tokens: VecDeque<Token>) -> (Vec<Self>,Vec<String>) {
        //println!("{:?}", tokens); // Debugging
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut variables:Vec<String> = Vec::new();
        let start_index:usize;
        let end_index:usize;
        let mut loop_operations:VecDeque<Token> = VecDeque::new();
        if tokens.iter().position(|x| x.to_string().contains(".loop")).is_some() {
            start_index = tokens.iter().position(|x| x.to_string().contains(".loop")).unwrap();
            // println!("Index of loop: {}", start_index); // Debugging
            end_index = tokens.iter().position(|x| x.to_string().contains(".endloop")).unwrap();
            // println!("Index of endloop: {}", end_index); // Debugging
            loop_operations = tokens.drain(start_index+1..end_index).collect::<VecDeque<Token>>();
            // println!("{:?}", loop_operations); // Debugging
        }
        for token in tokens {
            match token {
                Token::Operation(s) => {
                    Self::process_operation(s, &mut instructions);
                },
                Token::Label(s) => {
                    match s.as_str() {
                        "#data" => println!("Data section"),
                        "#code" => println!("Main section"),
                        _ => println!("Label: {}", s)
                    }
                },
                Token::Variable(s) => {
                    variables.push(s.to_string());
                },
                Token::Directive(s) => {
                    let mut parts = s.split_whitespace();
                    let name = parts.next().unwrap();
                    let mut loop_count: i64 = 0;
                    match name {
                        ".main" => println!("Main program"),
                        ".end" => println!("End of program"),
                        ".loop" => {
                            println!("Loop start");
                            for part in parts {
                                if part.parse::<i64>().is_ok() {
                                    loop_count= part.parse::<i64>().unwrap();
                                }
                            }
                            for _ in 0..loop_count {
                                for op in &loop_operations{
                                    Self::process_operation(op.to_string(), &mut instructions);
                                }
                            }
                        },
                        ".endloop" => {
                            println!("End of loop")
                        },
                        _ => println!("Directive: {}", s)
                    }
                },
                _ => {}
            }
        }
        (instructions,variables)
    }

    pub fn process_operation(op: String, instr: &mut Vec<Instruction>){
        let mut operands = VecDeque::new();
        let mut parts = op.split_whitespace();
        let operation= Operation::from(parts.next().unwrap());
        for part in parts {
            if part.parse::<i64>().is_ok() {
                operands.push_back(Field::Integer(part.parse::<i64>().unwrap()));
            } else {
                operands.push_back(Field::String(part.to_string()));
            }
        }
        instr.push(Instruction::new(operation, operands));
    }

    pub fn execute_instruction(mut self, results: &mut VecDeque<Field>, variables: &Vec<Variable>) {
        match self.operation {
            Operation::Add => {
                let a = self.operands.pop_front().unwrap();
                let b = self.operands.pop_front().unwrap();
                match (a, b) {
                    (Field::Integer(a), Field::Integer(b)) => results.push_back(Field::Integer(a + b)),
                    (Field::String(a), Field::Integer(b)) => {
                        for var in variables {
                            if var.name == a {
                                results.push_back(Field::Integer(var.value.to_int().unwrap() + b));
                            }
                        }
                    },
                    (Field::Integer(a), Field::String(b)) => {
                        for var in variables {
                            if var.name == b {
                                results.push_back(Field::Integer(a + var.value.to_int().unwrap()));
                            }
                        }
                    },
                    _ => panic!("Invalid operands")
                }
            },
            Operation::Sub => {
                let a = self.operands.pop_front().unwrap();
                let b = self.operands.pop_front().unwrap();
                match (a, b) {
                    (Field::Integer(a), Field::Integer(b)) => results.push_back(Field::Integer(a - b)),
                    (Field::String(a), Field::Integer(b)) => {
                        for var in variables {
                            if var.name == a {
                                results.push_back(Field::Integer(var.value.to_int().unwrap() - b));
                            }
                        }
                    },
                    (Field::Integer(a), Field::String(b)) => {
                        for var in variables {
                            if var.name == b {
                                results.push_back(Field::Integer(a - var.value.to_int().unwrap()));
                            }
                        }
                    },
                    _ => panic!("Invalid operands")
                }
            },
            Operation::Mul => {
                let a = self.operands.pop_front().unwrap();
                let b = self.operands.pop_front().unwrap();
                match (a, b) {
                    (Field::Integer(a), Field::Integer(b)) => results.push_back(Field::Integer(a * b)),
                    (Field::String(a), Field::Integer(b)) => {
                        for var in variables {
                            if var.name == a {
                                results.push_back(Field::Integer(var.value.to_int().unwrap() * b));
                            }
                        }
                    },
                    (Field::Integer(a), Field::String(b)) => {
                        for var in variables {
                            if var.name == b {
                                results.push_back(Field::Integer(a * var.value.to_int().unwrap()));
                            }
                        }
                    },
                    _ => panic!("Invalid operands")
                }
            },
            Operation::Div => {
                let a = self.operands.pop_front().unwrap();
                let b = self.operands.pop_front().unwrap();
                match (a, b) {
                    (Field::Integer(a), Field::Integer(b)) => results.push_back(Field::Integer(a / b)),
                    (Field::String(a), Field::Integer(b)) => {
                        for var in variables {
                            if var.name == a {
                                results.push_back(Field::Integer(var.value.to_int().unwrap() / b));
                            }
                        }
                    },
                    (Field::Integer(a), Field::String(b)) => {
                        for var in variables {
                            if var.name == b {
                                results.push_back(Field::Integer(a / var.value.to_int().unwrap()));
                            }
                        }
                    },
                    _ => panic!("Invalid operands")
                }
            },
            Operation::Print => {
                for operand in self.operands {
                    match operand {
                        Field::Integer(i) => print!("{} ", i),

                        Field::String(s) => {
                            if variables.iter().any(|var| var.name == s) {
                                for var in variables {
                                    if var.name == s {
                                        print!("{} ", var.value.to_string());
                                    }
                                }
                            } else {
                                print!("{} ", s.replace("\"", ""));
                            }
                        }
                    }
                }
                println!();
            }
        }
    }

}