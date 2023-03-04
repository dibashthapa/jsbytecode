use crate::error::{LoxResult, Error, LoxErrors};
use crate::stmt::{StmtVisitor, ExpressionStmt, PrintStmt, Stmt};
use crate::token::Token;
use crate::token_type::TokenType::{
    Bang, Greater, GreaterEqual, Less, LessEqual, Minus, Plus, Slash, Star, BangEqual, EqualEqual
};
use crate::{
    expr::ExprVisitor,
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    value::Value,
};

pub struct Intrepreter;

type Literal = Option<Value>;

impl Intrepreter {
    pub fn new () -> Self {
        Self
    }

    fn evaluate(&mut self, expr: &Expr) -> LoxResult<Literal> {
        expr.accept(self)
    }

    pub fn intrepret(&mut self, statements: &[Stmt]) -> LoxResult<()> {
        for stmt in statements.iter() {
            println!("statement is {:#?}", stmt);
            self.execute(stmt)?
        }
        Ok(())
    }

    fn execute(&mut self , stmt: &Stmt) -> LoxResult<()>{
        stmt.accept(self)
    }
}

fn is_truthy(object: &Value) -> bool {
        if *object == Value::Nil {
            false
        } else {
            true
        }
    }

fn is_equal(a: &Literal, b:&Literal) -> bool {
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
        _ => Err(LoxErrors::RunTimeException(Error::new(operator.line, "Operand must be a number".to_string())))
    }
}

fn check_number_operands(operator: &Token, left: &Value, right: &Value) -> LoxResult<()> {
    match (left, right)  {
        (Value::Number(_), Value::Number(_)) => Ok(()),
        _ => Err(LoxErrors::RunTimeException(Error::new(operator.line, "Operands must be numbers".to_string())))
    }
}

impl ExprVisitor<LoxResult<Literal>>   for Intrepreter {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> LoxResult<Literal> {
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
                },
                _ => Ok(Some(Value::Nil))
            },
            _ => Ok(None)
        }
    }
fn visit_binary_exp(&mut self, expr: &BinaryExpr) -> LoxResult<Literal> {
        let left = self.evaluate(&expr.left)?.unwrap();
        let right = self.evaluate(&expr.right)?.unwrap();

        match expr.operator.type_ {
            Minus => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left - right))
            },
            Slash => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left / right))
            },
            Star => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left * right))
            },
            Plus => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left + right))
            },
            Greater => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(Value::Boolean(left > right)))
            },
            GreaterEqual => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(Value::Boolean(left >= right)))
            },
            Less => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(Value::Boolean(left < right)))
            },
            LessEqual => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(Value::Boolean(left <= right)))
            },
            BangEqual => Ok(Some(Value::Boolean(!is_equal(&Some(left), &Some(right))))),
            EqualEqual => Ok(Some(Value::Boolean(is_equal(&Some(left), &Some(right))))),
            _ => Err(LoxErrors::RunTimeException(Error::new(expr.operator.line, "Operands must be two numbers or two string".to_string())))
        }
    }
}

impl StmtVisitor<LoxResult<()>> for Intrepreter {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> LoxResult<()> {
        self.evaluate(&stmt.expression)?.unwrap();
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> LoxResult<()> {
        let value = self.evaluate(&stmt.expression)?.unwrap();
        println!("{}", value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_literal_num(num: f64) -> Box<Expr>  {
        Box::new(Expr::Literal(LiteralExpr { value: Some(Value::Number(num)) }))
    }

    #[test]
    fn test_unary_minus(){
        let mut terp = Intrepreter {};
        let unary = UnaryExpr {
            operator: Token::new(Minus , "-", None, 1),
            right: make_literal_num(123.0)
        };

        let result = terp.visit_unary_expr(&unary);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Number(-123.0))));
    }

    #[test]
    fn test_unary_not() {
        let mut terp = Intrepreter {};
        let unary = UnaryExpr {
            operator: Token::new(Bang, "!", None, 1),
            right: Box::new(Expr::Literal(LiteralExpr { value: Some(Value::Boolean(true)) }))
        };
        let result = terp.visit_unary_expr(&unary);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Boolean(false))));
    }

    #[test]
    fn test_binary_sub(){
        let mut terp = Intrepreter{};
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: Some(Value::Number(100.0)) })),
            operator: Token::new(Minus , "-", None , 1),
            right: Box::new(Expr::Literal(LiteralExpr { value: Some(Value::Number(50.0)) }))
        };

        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Number(50.0))) );
    }

    #[test]
    fn test_binary_add(){
        let mut terp = Intrepreter{};
        let binary_expr = BinaryExpr {
            left: make_literal_num(100.0),
            operator: Token::new(Plus , "+", None , 1),
            right: make_literal_num(200.0)
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Number(300.0))) );
    }

    #[test]
    fn test_binary_mul(){
        let mut terp = Intrepreter{};
        let binary_expr = BinaryExpr {
            left: make_literal_num(10.0),
            operator: Token::new(Star , "*", None , 1),
            right: make_literal_num(20.0)
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Number(200.0))) );
    }

    #[test]
    fn test_binary_equals(){
        let mut terp = Intrepreter{};
        let binary_expr = BinaryExpr {
            left: make_literal_num(15.0),
            operator: Token::new(EqualEqual, "==", None , 1),
            right: make_literal_num(10.0)
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Boolean(false))) );
    }


    #[test]
    fn test_binary_div(){
        let mut terp = Intrepreter{};
        let binary_expr = BinaryExpr {
            left: make_literal_num(50.0),
            operator: Token::new(Slash , "/", None , 1),
            right: make_literal_num(10.0)
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Number(5.0))) );
    }

    #[test]
    fn test_binary_greater(){
        let mut terp = Intrepreter{};
        let binary_expr = BinaryExpr {
            left: make_literal_num(50.0),
            operator: Token::new(Greater , ">", None , 1),
            right: make_literal_num(10.0)
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Boolean(true))) );
    }

    #[test]
    fn test_binary_smaller(){
        let mut terp = Intrepreter{};
        let binary_expr = BinaryExpr {
            left: make_literal_num(5.0),
            operator: Token::new(Less , "<", None , 1),
            right: make_literal_num(10.0)
        };
        let result = terp.visit_binary_exp(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Some(Value::Boolean(true))) );
    }

}

