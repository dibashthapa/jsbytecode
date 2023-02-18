use crate::{token_type::TokenType, value::Value};

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    pub literal: Option<Value>,
    line: usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, literal: Option<Value>, line: usize) -> Self {
        Self {
            type_,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }

    pub fn type_(&self) -> String {
        self.type_.to_string()
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn literal(&self) -> &Option<Value> {
        &self.literal
    }

    pub fn line(&self) -> usize {
        self.line
    }
}
