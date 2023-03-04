use crate::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Print(PrintStmt),
    Expression(ExpressionStmt)
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Print(v) => visitor.visit_print_stmt(v),
            Stmt::Expression(v) => visitor.visit_expression_stmt(v),
        }
    }
    
}

pub trait StmtVisitor<T> {
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> T;
    fn visit_expression_stmt(&mut self, expr: &ExpressionStmt) -> T;
}

#[derive(Debug)]
pub struct PrintStmt {
    pub expression: Expr
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub expression: Expr
}

