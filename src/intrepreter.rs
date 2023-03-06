use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{
    AssignExpr, BinaryExpr, BlockStmt, Expr, ExpressionStmt, GroupingExpr, LiteralExpr, PrintStmt,
    Stmt, UnaryExpr, VarStmt, VariableExpr, VisitorExpr, VisitorStmt,
};
use crate::environment::Environment;
use crate::error::{Error, LoxErrors, LoxResult};
use crate::token::Token;
use crate::token_type::TokenType::{
    Bang, BangEqual, EqualEqual, Greater, GreaterEqual, Less, LessEqual, Minus, Plus, Slash, Star,
};
use crate::tools::*;
use crate::value::Value;

pub struct Intrepreter {
    environment: Rc<RefCell<Environment>>,
}

type Literal = Option<Value>;

impl Intrepreter {
    fn new(environment: Rc<RefCell<Environment>>) -> Self {
        Self { environment }
    }
    fn evaluate(&mut self, expr: &Expr) -> LoxResult<Literal> {
        expr.accept(self)
    }

    pub fn intrepret(&mut self, statements: &[Stmt]) -> LoxResult<()> {
        for stmt in statements.iter() {
            self.execute(stmt)?
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> LoxResult<()> {
        stmt.accept(self)
    }

    fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Rc<RefCell<Environment>>,
    ) -> LoxResult<()> {
        let mut interpreter = Intrepreter::new(environment);
        statements
            .iter()
            .try_for_each(|stmt| interpreter.execute(stmt))?;
        Ok(())
    }
}

impl Default for Intrepreter {
    fn default() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::default())),
        }
    }
}

fn is_truthy(object: &Value) -> bool {
    if *object == Value::Nil {
        false
    } else {
        true
    }
}

fn is_equal(a: &Literal, b: &Literal) -> bool {
    if a.is_none() && b.is_none() {
        false
    } else if a.is_none() {
        false
    } else {
        a.eq(b)
    }
}

fn check_number_and_operand(operator: &Token, operand: &Value) -> LoxResult<()> {
    match operand {
        Value::Number(_) => Ok(()),
        _ => Err(LoxErrors::RunTimeException(Error::new(
            operator.line,
            "Operand must be a number".to_string(),
        ))),
    }
}

fn check_number_operands(operator: &Token, left: &Value, right: &Value) -> LoxResult<()> {
    match (left, right) {
        (Value::Number(_), Value::Number(_)) => Ok(()),
        _ => Err(LoxErrors::RunTimeException(Error::new(
            operator.line,
            "Operands must be numbers".to_string(),
        ))),
    }
}

impl VisitorExpr for Intrepreter {
    type Result = LoxResult<Literal>;

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Self::Result {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(expr.name.clone(), value.clone())?;
        Ok(value)
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Self::Result {
        self.environment.borrow_mut().get(expr.name.clone())
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Self::Result {
        Ok(expr.value.to_owned())
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> LoxResult<Literal> {
        self.evaluate(&*expr.expression)
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> LoxResult<Literal> {
        let right = self.evaluate(&expr.right)?.unwrap();

        match expr.operator.type_ {
            Bang => Ok(Some(Value::Boolean(!is_truthy(&right)))),
            Minus => match right {
                Value::Number(n) => {
                    check_number_and_operand(&expr.operator, &right)?;
                    Ok(Some(Value::Number(-n)))
                }
                _ => Ok(Some(Value::Nil)),
            },
            _ => Ok(None),
        }
    }
    fn visit_binary_exp(&mut self, expr: &BinaryExpr) -> LoxResult<Literal> {
        let left = self.evaluate(&expr.left)?.unwrap();
        let right = self.evaluate(&expr.right)?.unwrap();

        match expr.operator.type_ {
            Minus => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left - right))
            }
            Slash => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left / right))
            }
            Star => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left * right))
            }
            Plus => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left + right))
            }
            Greater => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(Value::Boolean(left > right)))
            }
            GreaterEqual => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(Value::Boolean(left >= right)))
            }
            Less => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(Value::Boolean(left < right)))
            }
            LessEqual => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(Value::Boolean(left <= right)))
            }
            BangEqual => Ok(Some(Value::Boolean(!is_equal(&Some(left), &Some(right))))),
            EqualEqual => Ok(Some(Value::Boolean(is_equal(&Some(left), &Some(right))))),
            _ => Err(LoxErrors::RunTimeException(Error::new(
                expr.operator.line,
                "Operands must be two numbers or two string".to_string(),
            ))),
        }
    }
}

/*
     program        → statement* EOF ;
     statement      → exprStmt
                    | printStmt ;
     exprStmt       → expression ";" ;
     printStmt      → "print" expression ";" ;
*/

impl VisitorStmt for Intrepreter {
    type Result = LoxResult<()>;
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Self::Result {
        let value = self.evaluate(&stmt.expression)?.unwrap();
        println!("{value}");
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Self::Result {
        let value = self.evaluate(&stmt.expression)?.unwrap();
        println!("{value}");
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Self::Result {
        let mut value = None;

        if let Some(initializer) = &stmt.initializer {
            value = self.evaluate(&initializer)?;
        }
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Self::Result {
        self.execute_block(&stmt.statements, self.environment.clone())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_literal_num(num: f64) -> Box<Expr> {
        Box::new(Expr::LiteralExpr(LiteralExpr {
            value: Some(Value::Number(num)),
        }))
    }

    #[test]
    fn test_unary_minus() {
        let mut terp = Intrepreter::default();
        let unary = UnaryExpr {
            operator: Token::new(Minus, "-", None, 1),
            right: make_literal_num(123.0),
        };

        let result = terp.visit_unary_expr(&unary);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Number(-123.0))));
    }

    #[test]
    fn test_unary_not() {
        let mut terp = Intrepreter::default();
        let unary = UnaryExpr {
            operator: Token::new(Bang, "!", None, 1),
            right: Box::new(Expr::LiteralExpr(LiteralExpr {
                value: Some(Value::Boolean(true)),
            })),
        };
        let result = terp.visit_unary_expr(&unary);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Boolean(false))));
    }

    #[test]
    fn test_binary_sub() {
        let mut terp = Intrepreter::default();
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::LiteralExpr(LiteralExpr {
                value: Some(Value::Number(100.0)),
            })),
            operator: Token::new(Minus, "-", None, 1),
            right: Box::new(Expr::LiteralExpr(LiteralExpr {
                value: Some(Value::Number(50.0)),
            })),
        };

        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Number(50.0))));
    }

    #[test]
    fn test_binary_add() {
        let mut terp = Intrepreter::default();
        let binary_expr = BinaryExpr {
            left: make_literal_num(100.0),
            operator: Token::new(Plus, "+", None, 1),
            right: make_literal_num(200.0),
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Number(300.0))));
    }

    #[test]
    fn test_binary_mul() {
        let mut terp = Intrepreter::default();
        let binary_expr = BinaryExpr {
            left: make_literal_num(10.0),
            operator: Token::new(Star, "*", None, 1),
            right: make_literal_num(20.0),
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Number(200.0))));
    }

    #[test]
    fn test_binary_equals() {
        let mut terp = Intrepreter::default();
        let binary_expr = BinaryExpr {
            left: make_literal_num(15.0),
            operator: Token::new(EqualEqual, "==", None, 1),
            right: make_literal_num(10.0),
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Boolean(false))));
    }

    #[test]
    fn test_binary_div() {
        let mut terp = Intrepreter::default();
        let binary_expr = BinaryExpr {
            left: make_literal_num(50.0),
            operator: Token::new(Slash, "/", None, 1),
            right: make_literal_num(10.0),
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Number(5.0))));
    }

    #[test]
    fn test_binary_greater() {
        let mut terp = Intrepreter::default();
        let binary_expr = BinaryExpr {
            left: make_literal_num(50.0),
            operator: Token::new(Greater, ">", None, 1),
            right: make_literal_num(10.0),
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Boolean(true))));
    }

    #[test]
    fn test_binary_smaller() {
        let mut terp = Intrepreter::default();
        let binary_expr = BinaryExpr {
            left: make_literal_num(5.0),
            operator: Token::new(Less, "<", None, 1),
            right: make_literal_num(10.0),
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Boolean(true))));
    }
}
