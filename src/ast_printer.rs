use crate::expr::{Expr, ExprVisitor};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self
    }
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }
    fn parenthesize(&mut self, name: String, exprs: &[Expr]) -> String {
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
    fn visit_binary_exp(&mut self, expr: &crate::expr::BinaryExpr) -> String {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &crate::expr::GroupingExpr) -> String {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &crate::expr::UnaryExpr) -> String {
        todo!()
    }

    fn visit_literal_expr(&mut self, expr: &crate::expr::LiteralExpr) -> String {
        todo!()
    }
}
