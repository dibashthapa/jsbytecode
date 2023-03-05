use crate::ast::{VisitorExpr, VisitorStmt};

pub trait AstNode {
    fn accept<K: VisitorExpr>(&self, visitor: &mut K) -> K::Result;
}

pub trait AstStmt {
    fn accept<K: VisitorStmt>(&self, visitor: &mut K) -> K::Result;
}
