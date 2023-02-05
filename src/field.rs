use std::fmt::Display;

#[derive(Debug)]
pub enum Field {
    Integer(i64),
    String(String),
}

impl Field {
    pub fn to_int(&self) -> Option<i64> {
        /// This function takes a field and returns an integer if the field is an integer.
        match self {
            &Field::Integer(i) => Some(i),
            _ => None
        }
    }

    pub fn to_s(&self) -> Option<String> {
        /// This function takes a field and returns a string if the field is a string.
        match self {
            &Field::String(ref s) => Some(s.to_string()),
            _ => None
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Field::Integer(i) => write!(f, "{}", i),
            &Field::String(ref s) => write!(f, "{}", s)
        }
    }
}