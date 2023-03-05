use crate::tools::{AstNode, AstStmt};
use crate::{define_ast, token::Token, value::Value as LiteralEnum};

define_ast!(
    AstNode,
    VisitorExpr,
    Expr,
    [
        BinaryExpr {
            left: Box<Expr>,
            operator: Token,
            right: Box<Expr>
        },
        visit_binary_exp
    ],
    [
        GroupingExpr {
            expression: Box<Expr>
        },
        visit_grouping_expr
    ],
    [
        LiteralExpr {
            value: Option<LiteralEnum>
        },
        visit_literal_expr
    ],
    [
        UnaryExpr {
            operator: Token,
            right: Box<Expr>
        },
        visit_unary_expr
    ],
);

define_ast!(
    AstStmt,
    VisitorStmt,
    Stmt,
    [ExpressionStmt { expression: Expr }, visit_expression_stmt],
    [PrintStmt { expression: Expr }, visit_print_stmt],
);
