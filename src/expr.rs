use crate::token::Token;
use crate::value::Value;

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(v) => visitor.visit_binary_exp(v),
            Expr::Grouping(v) => visitor.visit_grouping_expr(v),
            Expr::Literal(v) => visitor.visit_literal_expr(v),
            Expr::Unary(v) => visitor.visit_unary_expr(v),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary_exp(&mut self, expr: &BinaryExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Option<Value>,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

