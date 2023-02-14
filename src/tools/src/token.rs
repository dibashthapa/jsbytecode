use std::fmt;

#[derive(Clone, Debug)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Nil => write!(f, "nil"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Boolean(s), Value::Boolean(o)) => s == o,
            (Value::Nil, Value::Nil) => true,
            (Value::Number(s), Value::Number(o)) => s == o,
            (Value::String(s), Value::String(o)) => s == o,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    Equal,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    type_: TokenType,
    pub lexeme: String,
    literal: Option<Value>,
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

    pub fn literal(&self) -> &Option<Value> {
        &self.literal
    }

    pub fn line(&self) -> usize {
        self.line
    }
}
