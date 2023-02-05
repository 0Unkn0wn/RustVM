use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Print
}

impl From<&str> for Operation {
    fn from(str: &str) -> Self{
        match str {
            "add" => Operation::Add,
            "sub" => Operation::Sub,
            "mul" => Operation::Mul,
            "div" => Operation::Div,
            "print" => Operation::Print,
            _ => panic!("Invalid instruction")
        }
    }
}

impl From<Operation> for String{
    fn from(instruction: Operation) -> Self{
        match instruction {
            Operation::Add => "add".to_string(),
            Operation::Sub => "sub".to_string(),
            Operation::Mul => "mul".to_string(),
            Operation::Div => "div".to_string(),
            Operation::Print => "print".to_string()
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "add"),
            Operation::Sub => write!(f, "sub"),
            Operation::Mul => write!(f, "mul"),
            Operation::Div => write!(f, "div"),
            Operation::Print => write!(f, "print")
        }
    }
}

impl Operation {
    pub fn execute(&self, stack: &mut VecDeque<i64>) {
        /// This function takes an operation and a stack and executes the operation.
        /// # Arguments
        /// * `self` - The operation to execute
        /// * `stack` - The stack to execute the operation on
        /// # Panics
        /// This function panics if the stack is empty.
        let a = stack.pop_back().unwrap();
        let b = stack.pop_back().unwrap();
        match self {
            Operation::Add => stack.push_back(a + b),
            Operation::Sub => stack.push_back(a - b),
            Operation::Mul => stack.push_back(a * b),
            Operation::Div => stack.push_back(a / b),
            Operation::Print => println!("{}", a)
        }
    }
}