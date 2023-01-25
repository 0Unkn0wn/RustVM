use std::fmt::{Display, Formatter};
use crate::field::Field;

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: Field
}

impl Variable {
    pub fn new(name: String, value: Field) -> Self { Variable{ name, value } }

    pub fn from_vec_string(vec_string: Vec<String>) -> Vec<Self> {
        let mut variables: Vec<Variable> = Vec::new();
        for string in vec_string {
            let mut parts = string.split_whitespace();
            let name = parts.next().unwrap().to_string().replace("@", "");
            let value = parts.next().unwrap();
            if value.parse::<i64>().is_ok() {
                let value = Field::Integer(value.parse::<i64>().unwrap());
                variables.push(Variable::new(name, value));
            }
            else {
                let value = Field::String(value.to_string().replace("\"", ""));
                variables.push(Variable::new(name, value));
            }
        }
        variables
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.name, self.value)
    }
}