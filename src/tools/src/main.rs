mod ast;
mod ast_printer;
mod expr;
use ast_printer::AstPrinter;
use expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use token::{Token, TokenType, Value};
// mod expr;
mod token;
fn main() {
    let expr = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Unary(UnaryExpr {
            operator: Token::new(TokenType::Minus, "-", None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Value::Number(123.0)),
            })),
        })),
        operator: Token::new(TokenType::Star, "*", None, 1),
        right: Box::new(Expr::Grouping(GroupingExpr {
            expression: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Value::Number(45.67)),
            })),
        })),
    });

    println!("{}", AstPrinter::new().print(&expr))

    // let env: Vec<String> = std::env::args().collect();
    // if env.len() <= 1 {
    //     eprintln!("Usage: generate_ast <output directory>");
    //     exit(64);
    // }

    // let output_dir = env.get(1).unwrap().to_string();

    // define_ast(
    //     output_dir,
    //     "Expr",
    //     &[
    //         "Binary   : left Box<Expr>, operator Token, right Box<Expr>",
    //         "Grouping : expression Box<Expr>",
    //         "Literal  : value Option<Value>",
    //         "Unary    : operator Token , right Box<Expr>",
    //     ],
    // )?;
    // Ok(())
}
