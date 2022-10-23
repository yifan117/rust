use crate::tokeniser::{Token, Keyword, Symbol};


#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment(Assignment), // Declaration and assignment are the same
    // If(If),
    // Loop(Loop),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub identifier: String,
    pub value: Value,
}

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let mut statements: Vec<Statement> = Vec::new();
    let mut tokens = tokens.iter().peekable();

    while let Some(token) = tokens.next() {
        println!("this token: {:?}", token);
        match token {
            Token::Keyword(Keyword::Let) => {
                let identifier = match tokens.next() {
                    Some(Token::Identifier(identifier)) => identifier,
                    _ => panic!("Expected identifier after let"),
                };

                tokens.next(); // Skip the assign symbol

                let value = match tokens.next() {
                    Some(Token::Int(value)) => Value::Int(*value),
                    Some(Token::Float(value)) => Value::Float(*value),
                    Some(Token::String(value)) => Value::String(value.to_string()),
                    Some(Token::Bool(value)) => Value::Bool(*value),
                    _ => panic!("Expected value after identifier"),
                };

                statements.push(Statement::Assignment(Assignment {
                    identifier: identifier.to_string(),
                    value,
                }));
            },
            Token::Symbol(Symbol::Semicolon) => {
                tokens.next();
            },
            _ => panic!("Expected let"),
        }
    }

    statements
}