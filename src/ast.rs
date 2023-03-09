use crate::tools::{AstNode, AstStmt};
use crate::{define_ast, token::Token, value::Value as LiteralEnum};

// expression     → assignment ;
// assignment     → IDENTIFIER "=" assignment
//                | logic_or ;
// logic_or       → logic_and ( "or" logic_and )* ;
// logic_and      → equality ( "and" equality )* ;

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
        AssignExpr {
            name: Token,
            value: Box<Expr>
        },
        visit_assign_expr
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
    [
        VariableExpr {
            name: Token
        },
        visit_variable_expr
    ],
    [
        LogicalExpr {
            left: Box<Expr>,
            operator: Token,
            right: Box<Expr>
        },
        visit_logical_expr
    ],
);

// statement      → exprStmt
//                | ifStmt
//                | printStmt
//                | forStmt
//                | whileStmt
//                | block ;
//
// whileStmt      → "while" "(" expression ")" statement ;
//
// ifStmt         → "if" "(" expression ")" statement
//                ( "else" statement )? ;

// forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
//                  expression? ";"
//                  expression? ")" statement ;

define_ast!(
    AstStmt,
    VisitorStmt,
    Stmt,
    [BlockStmt { statements: Vec<Stmt> }, visit_block_stmt],
    [ExpressionStmt { expression: Expr }, visit_expression_stmt],
    [PrintStmt { expression: Expr }, visit_print_stmt],
    [VarStmt { name: Token , initializer: Option<Expr> }, visit_var_stmt],
    [IfStmt { condition: Expr , then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>}, visit_if_stmt],
    [WhileStmt { condition: Expr, body: Box<Stmt>}, visit_while_stmt],
);
