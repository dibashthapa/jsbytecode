use crate::{
    error::{Error, LoxErrors},
    token::Token,
    token_type::TokenType,
};

type LoxResult<T> = Result<T, LoxErrors>;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        for ttype in token_types {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().type_ == TokenType::EOF
    }

    fn is_at_end(&self) -> bool {
        self.peek().type_ == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> LoxResult<&Token> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    fn error(&self, peek: &Token, message: &str) -> LoxErrors {
        LoxErrors::ParseError(Error::new(peek.line, message.to_string()))
    }
}
