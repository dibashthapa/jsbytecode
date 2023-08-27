use crate::{
    ast::{
        Assign, Binary, BlockStmt, Expr, ExpressionStmt, Grouping, IfStmt, Literal, Logical,
        PrintStmt, Stmt, Unary, VarStmt, Variable, WhileStmt,
    },
    error::{Error, LoxErrors, LoxResult},
    token::Token,
    token_type::TokenType::{self, *},
    value::Value,
};

pub trait ParseExpr {
    fn expression(&mut self) -> LoxResult<Expr>;
    fn equality(&mut self) -> LoxResult<Expr>;
    fn assignment(&mut self) -> LoxResult<Expr>;
    fn comparison(&mut self) -> LoxResult<Expr>;
    fn term(&mut self) -> LoxResult<Expr>;
    fn factor(&mut self) -> LoxResult<Expr>;
    fn unary(&mut self) -> LoxResult<Expr>;
    fn primary(&mut self) -> LoxResult<Expr>;
    fn or(&mut self) -> LoxResult<Expr>;
    fn and(&mut self) -> LoxResult<Expr>;
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

        false
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
        self.peek().type_ == TokenType::Eof
    }

    fn statement(&mut self) -> LoxResult<Stmt> {
        if self.match_token(&[If]) {
            return self.if_statment();
        }
        if self.match_token(&[Print]) {
            return self.print_statement();
        }

        if self.match_token(&[For]) {
            return self.for_statement();
        }

        if self.match_token(&[While]) {
            return self.while_statement();
        }

        if self.match_token(&[LeftBrace]) {
            return Ok(Stmt::BlockStmt(BlockStmt {
                statements: self.block()?,
            }));
        }

        self.expression_statement()
    }

    fn while_statement(&mut self) -> LoxResult<Stmt> {
        self.consume(LeftParen, "Expect '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(RightParen, "Expect ')' after condition")?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::WhileStmt(WhileStmt { condition, body }))
    }

    fn for_statement(&mut self) -> LoxResult<Stmt> {
        self.consume(LeftParen, "Expect '(' after 'for'")?;
        let initializer;
        if self.match_token(&[Semicolon]) {
            initializer = None
        } else if self.match_token(&[Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition = None;

        if !self.check(&Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(Semicolon, "Expect ';' after loop condition")?;

        let mut increment = None;

        if !self.check(&RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(RightParen, "Expect ')' after for clauses")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::BlockStmt(BlockStmt {
                statements: vec![
                    body,
                    Stmt::ExpressionStmt(ExpressionStmt {
                        expression: increment,
                    }),
                ],
            });
        }

        if let Some(condition) = condition {
            body = Stmt::WhileStmt(WhileStmt {
                condition,
                body: Box::new(body),
            });
            if let Some(initializer) = initializer {
                body = Stmt::BlockStmt(BlockStmt {
                    statements: vec![initializer, body],
                });
            }
        }
        Ok(body)
    }

    fn if_statment(&mut self) -> LoxResult<Stmt> {
        self.consume(LeftParen, "Expect '(' after 'if' ")?;
        let condition = self.expression()?;
        self.consume(RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;

        if self.match_token(&[Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::IfStmt(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn block(&mut self) -> LoxResult<Vec<Stmt>> {
        let mut statements = vec![];

        while !self.check(&RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?)
        }

        self.consume(RightBrace, "Expect'}' after block.")?;

        Ok(statements)
    }

    fn expression_statement(&mut self) -> LoxResult<Stmt> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::ExpressionStmt(ExpressionStmt { expression: expr }))
    }

    fn print_statement(&mut self) -> LoxResult<Stmt> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::PrintStmt(PrintStmt { expression: expr }))
    }

    pub fn parse(&mut self) -> LoxResult<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> LoxResult<Stmt> {
        if self.match_token(&[Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> LoxResult<Stmt> {
        let name = self.consume(Identifier, "Expect variable name")?.clone();
        let mut initializer = None;

        if self.match_token(&[Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(Semicolon, "Expect ; after variable declaration")?;

        Ok(Stmt::VarStmt(VarStmt { initializer, name }))
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
        self.assignment()
    }

    fn assignment(&mut self) -> LoxResult<Expr> {
        let expr = self.or()?;

        if self.match_token(&[Equal]) {
            let equals = self.previous().to_owned();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(Variable { name }) => {
                    return Ok(Expr::Assign(Assign {
                        name,
                        value: Box::new(value),
                    }));
                }
                _ => return Err(self.error(&equals, "Invalid assignment target")),
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> LoxResult<Expr> {
        let mut expr = self.and()?;

        while self.match_token(&[Or]) {
            let operator = self.previous().to_owned();
            let right = self.and()?;
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn and(&mut self) -> LoxResult<Expr> {
        let mut expr = self.equality()?;

        while self.match_token(&[And]) {
            let operator = self.previous().to_owned();
            let right = self.equality()?;
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn primary(&mut self) -> LoxResult<Expr> {
        if self.match_token(&[False]) {
            return Ok(Expr::Literal(Literal {
                value: Some(Value::Boolean(false)),
            }));
        }
        if self.match_token(&[True]) {
            return Ok(Expr::Literal(Literal {
                value: Some(Value::Boolean(true)),
            }));
        }
        if self.match_token(&[Nil]) {
            return Ok(Expr::Literal(Literal {
                value: Some(Value::Nil),
            }));
        }
        if self.match_token(&[Number, TokenType::String]) {
            return Ok(Expr::Literal(Literal {
                value: self.previous().literal.to_owned(),
            }));
        }
        if self.match_token(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Grouping {
                expression: Box::new(expr),
            }));
        }

        if self.match_token(&[Identifier]) {
            return Ok(Expr::Variable(Variable {
                name: self.previous().to_owned(),
            }));
        } else {
            Err(LoxErrors::ParseError(Error::new(
                self.previous().line,
                "Expect Expression.".to_string(),
            )))
        }
    }

    fn equality(&mut self) -> LoxResult<Expr> {
        let mut expr = self.comparison()?;

        while self.match_token(&[BangEqual, EqualEqual]) {
            let operator = self.previous().to_owned();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary {
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

            expr = Expr::Binary(Binary {
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
            expr = Expr::Binary(Binary {
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

            expr = Expr::Binary(Binary {
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

            return Ok(Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }
}
