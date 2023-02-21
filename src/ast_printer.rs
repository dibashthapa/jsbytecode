use crate::expr::{Expr, ExprVisitor, BinaryExpr, GroupingExpr, UnaryExpr, LiteralExpr};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self
    }
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

 fn parenthesize(&mut self, name: String, exprs: &[&Box<Expr>]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(&name);

        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');

        builder
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_exp(&mut self, expr: &BinaryExpr) -> String {
        self.parenthesize(expr.operator.lexeme.clone(), &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> String {
        self.parenthesize("group".to_string(), &[&expr.expression])
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> String {
        self.parenthesize(expr.operator.lexeme.clone(), &[&expr.right])
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> String {
        expr.value.as_ref().unwrap().to_string()
    }}
