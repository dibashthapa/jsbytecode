use crate::{
    expr::{BinaryExpr, Expr, LiteralExpr, UnaryExpr},
    token_type::TokenType,
    token::Token,
    value::Value, error::LoxError
};

type LoxResult<T> = Result<T, LoxError>;


pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(&mut self, tokens: Vec<Token>) {
        self.current = 0;
        self.tokens = tokens;
    }

    fn expression(self) -> LoxResult<Expr> {
        self.equality()
    }

    fn equality(self) -> LoxResult<Expr> {
        let expr = self.comparison();

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
        }

        expr
    }

    fn comparison(self) -> LoxResult<Expr> {
        let expr = self.term();

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(self) -> Expr {
        let expr = self.factor();

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = Box::new(self.factor());
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        expr
    }

    fn factor(self) -> LoxResult<Expr> {
        let expr = self.unary();

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = Box::new(self.unary());
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right,
            })
        }

        return expr;
    }

    fn unary(self) -> LoxResult<Expr> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = Box::new(self.unary());

            return Expr::Unary(UnaryExpr { operator, right });
        }

        return self.primary();
    }

    fn primary(self) -> LoxResult<Expr> {

        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Value::Boolean(false)),
            }));
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Value::Boolean(true)),
            }));
        }

        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Value::Nil),
            }));
        }

        if self.match_token(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return expr;
        } 
        else {
            return Err(LoxError {
                line: 1, 
                message: "Unexpected expresssion".to_string()
            });
        }

    }

    fn error(self, token: &Token , message: &str) -> LoxError {

    }

    fn consume(self, ttype: TokenType, message: &str) -> Token {
        if self.check(ttype) {
            return self.advance();
        }
        self.peek()
    }

    fn match_token(self, types: &[TokenType]) -> bool {
        for ttype in types {
            if self.check(*ttype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };

        return self.peek().type_() == ttype.to_string();
    }

    fn is_at_end(self) -> bool {
        self.peek().type_() == TokenType::EOF.to_string()
    }

    fn previous(self) -> Token {
        self.tokens[self.current - 1]
    }

    fn advance(self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(self) -> Token {
        self.tokens[self.current]
    }
}
