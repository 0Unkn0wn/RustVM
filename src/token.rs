use std::fs;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Token{
    Operation(String),
    Comment(String),
    Label(String),
    Directive(String),
    Variable(String),
    End
}

impl Token {
    pub fn tokenize_file(file_name: &str) -> VecDeque<Token> {
        /// This function takes a file name and returns a vector of tokens.
        /// # Arguments
        /// * `file_name` - A string slice that holds the file name
        let file = fs::read_to_string(file_name).expect("Unable to read file");
        let mut tokens = VecDeque::new();
        let mut lines = file.lines();
        let mut line = lines.next();
        while line.is_some() {
            // println!("Line: {:?}", line); //debug
            let text = line.unwrap().trim();
            // println!("Text: {:?}", text); //debug
            match text{
                "" => tokens.push_back(Token::End),
                _ => {
                    if text.starts_with(";") {
                        tokens.push_back(Token::Comment(text.to_string()));
                    } else if text.starts_with(".") {
                        tokens.push_back(Token::Directive(text.to_string()));
                    } else if text.starts_with("#") {
                        tokens.push_back(Token::Label(text.to_string()));
                    } else if text.starts_with("@") {
                        tokens.push_back(Token::Variable(text.to_string()));
                    } else {
                        tokens.push_back(Token::Operation(text.to_string()));
                    }
                }
            }
            line = lines.next();
        }
        tokens
    }
}

impl Display for Token{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Operation(s) => write!(f, "{}", s), //doesnt have the "Operation:" prefix like the others to make it easier to process the loop operations
            Token::Comment(s) => write!(f, "Comment: {}", s),
            Token::Label(s) => write!(f, "Label: {}", s),
            Token::Directive(s) => write!(f, "Directive: {}", s),
            Token::Variable(s) => write!(f, "Variable: {}", s),
            Token::End => write!(f, "End of line")
        }
    }
}
