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
