use std::fs;
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

#[derive(Debug)]
pub enum Field {
    Integer(i64),
    String(String),
}

impl Field {
    pub fn to_int(&self) -> Option<i64> {
        match self {
            &Field::Integer(i) => Some(i),
            _ => None
        }
    }

    pub fn to_s(&self) -> Option<String> {
        match self {
            &Field::String(ref s) => Some(s.to_string()),
            _ => None
        }
    }


}

#[derive(Debug)]
pub struct Instruction{
    pub operation: Operation,
    pub operands: VecDeque<Field>
}

impl Instruction {
    pub fn new(operation: Operation, operands: VecDeque<Field>) -> Self {
        Instruction{ operation, operands }
    }

    pub fn from_str(str: &str) -> Self {
        //println!("{:?}", str); // Debugging
        let mut parts = str.split_whitespace();
        let operation = Operation::from(parts.next().unwrap());
        let mut operands = VecDeque::new();
        for part in parts {
            if part.parse::<i64>().is_ok() {
                operands.push_back(Field::Integer(part.parse::<i64>().unwrap()));
            } else {
                operands.push_back(Field::String(part.to_string()));
            }
        }
        Instruction::new(operation, operands)
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


fn parse_file(file_name: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let file = fs::read_to_string(file_name).expect("Unable to read file");
    for line in file.lines() {
        instructions.push(Instruction::from_str(line));
    }
    instructions
}

fn calculate(instructions: Vec<Instruction>) -> VecDeque<Field>{
    let mut results: VecDeque<Field> = VecDeque::new();

    for instruction in instructions{
        instruction.execute(&mut results);
    }

    results
}

fn main() {

    let file_path = "src/test.txt";

    let code = parse_file(file_path);
    println!("{:?}", code);
    let tokens = calculate(code);
    println!("{:?}", tokens);

}