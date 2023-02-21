use crate::{
    error::{Error, LoxErrors, LoxResult},
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    token::Token,
    token_type::TokenType::{self, *},
    value::Value,
};

pub trait ParseExpr {
    fn expression(&mut self) -> LoxResult<Expr>;
    fn equality(&mut self) -> LoxResult<Expr>;
    fn comparison(&mut self) -> LoxResult<Expr>;
    fn term(&mut self) -> LoxResult<Expr>;
    fn factor(&mut self) -> LoxResult<Expr>;
    fn unary(&mut self) -> LoxResult<Expr>;
    fn primary(&mut self) -> LoxResult<Expr>;
}

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

    fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().type_ == *token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().type_ == TokenType::EOF
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(exp) => Some(exp),
            Err(_) => None,
        }
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

impl<'a> ParseExpr for Parser<'a> {
    fn expression(&mut self) -> LoxResult<Expr> {
        self.equality()
    }

    fn primary(&mut self) -> LoxResult<Expr> {
        if self.match_token(&[False]) {
            Ok(Expr::Literal(LiteralExpr {
                value: Some(Value::Boolean(false)),
            }))
        } else if self.match_token(&[True]) {
            Ok(Expr::Literal(LiteralExpr {
                value: Some(Value::Boolean(true)),
            }))
        } else if self.match_token(&[Nil]) {
            Ok(Expr::Literal(LiteralExpr {
                value: Some(Value::Nil),
            }))
        } else if self.match_token(&[Number, TokenType::String]) {
            Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal.to_owned(),
            }))
        } else if self.match_token(&[LeftParen]) {
            let expr = self.expression()?;
            match self.consume(RightParen, "Expect ')' after expression.") {
                Ok(_) => Ok(Expr::Grouping(GroupingExpr {
                    expression: Box::new(expr),
                })),
                Err(e) => Err(e),
            }
        } else {
            Err(LoxErrors::ParseError(Error::new(
                self.peek().line,
                "Expect Expression.".to_string(),
            )))
        }
    }

    fn equality(&mut self) -> LoxResult<Expr> {
        let mut expr = self.comparison()?;

        while self.match_token(&[BangEqual, EqualEqual]) {
            let operator = self.previous().to_owned();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> LoxResult<Expr> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().to_owned();
            let right = self.term()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn term(&mut self) -> LoxResult<Expr> {
        let mut expr = self.factor()?;

        while self.match_token(&[Minus, Plus]) {
            let operator = self.previous().to_owned();
            let right = Box::new(self.factor()?);
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn factor(&mut self) -> LoxResult<Expr> {
        let mut expr = self.unary()?;

        while self.match_token(&[Slash, Star]) {
            let operator = self.previous().to_owned();
            let right = self.unary()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn unary(&mut self) -> LoxResult<Expr> {
        if self.match_token(&[Bang, Minus]) {
            let operator = self.previous().to_owned();
            let right = self.unary()?;

            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        return self.primary();
    }
}
