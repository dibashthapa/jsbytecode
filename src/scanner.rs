use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::error::{Error, LoxErrors};
use crate::token::Token;
use crate::token_type::TokenType;
use crate::value::Value;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);
        keywords
    };
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::eof(self.line));

        self.tokens.to_vec()
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.is_matched('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.is_matched('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.is_matched('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.is_matched('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.is_matched('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.add_string(),
            ('0'..='9') => self.add_number(),
            ('a'..='z') | ('A'..='Z') | '_' => self.add_identifier(),
            _ => LoxErrors::ParseError(Error::new(self.line, "Unterminated string".to_string()))
                .report(),
        }
    }

    fn add_identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        if let Some(text) = self.source.get(self.start..self.current) {
            if let Some(type_) = KEYWORDS.get(text) {
                let type_: TokenType = type_.to_owned();
                self.add_token(type_)
            } else {
                self.add_token(TokenType::Identifier);
            }
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn add_number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let number = self.source[self.start..self.current].to_string();

        self.add_token_with_value(
            TokenType::Number,
            Some(Value::Number(number.parse::<f64>().unwrap())),
        );
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn add_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            };
            self.advance();
        }

        if self.is_at_end() {
            Error::new(self.line, "Unterminated string.".to_string());
        }

        self.advance();
        let value = self
            .source
            .get(self.start + 1..self.current - 1)
            .map(|s| s.to_string());

        if let Some(value) = value {
            self.add_token_with_value(TokenType::String, Some(Value::String(value)));
        } else {
            Error::new(self.line, "Invalid string slice".to_string());
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self, c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.current_char()
    }

    // gets the next character
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn current_char(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    fn add_token_with_value(&mut self, type_: TokenType, literal: Option<Value>) {
        let text = self.source.get(self.start..self.current).unwrap();
        let token = Token::new(type_, text, literal, self.line);
        self.tokens.push(token);
    }

    fn add_token(&mut self, type_: TokenType) {
        let text = self.source.get(self.start..self.current).unwrap();
        self.tokens.push(Token::new(type_, text, None, self.line));
    }

    fn is_matched(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.current_char() != expected {
            return false;
        }
        self.current += 1;
        true
    }
}
