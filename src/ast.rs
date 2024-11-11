use crate::{error::LoxErrors, token::Token, value::Value};

// expression     → assignment ;
// assignment     → IDENTIFIER "=" assignment
//                | logic_or ;
// logic_or       → logic_and ( "or" logic_and )* ;
// logic_and      → equality ( "and" equality )* ;

// The Expression enum holds references to its variants
#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Literal {
        value: Option<Value>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Variable {
        name: Token,
    },

    Assign {
        name: Token,
        value: Box<Expression>,
    },
    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
}

// Statement types follow the same pattern
#[derive(Debug)]
pub enum Statement {
    Block {
        statements: Vec<Statement>,
    },
    Expression {
        expression: Expression,
    },
    Print {
        expression: Expression,
    },
    Var {
        name: Token,
        initializer: Option<Expression>,
    },
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Box<Option<Statement>>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
}
