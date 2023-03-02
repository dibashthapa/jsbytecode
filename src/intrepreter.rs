use crate::error::{LoxResult, Error, LoxErrors};
use crate::token::Token;
use crate::token_type::TokenType::{
    Bang, Greater, GreaterEqual, Less, LessEqual, Minus, Plus, Slash, Star,EqualEqual, BangEqual
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

    pub fn intrepret(&mut self, expr: &Expr) -> LoxResult<()> {
        let value = self.evaluate(expr)?.unwrap();
        println!("{}", value.to_string());
        Ok(())
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

fn check_number_operands(operator: &Token, left: Value, right: Value) -> LoxResult<()> {
    match (left, right)  {
        (Value::Number(_), Value::Number(_)) => Ok(()),
        _ => Err(LoxErrors::RunTimeException(Error::new(operator.line, "Operands must be numbers".to_string())))
    }
}

impl ExprVisitor<LoxResult<Literal>> for Intrepreter {
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

        // TODO: Need to refactor this , when i get better at rust
        match expr.operator.type_ {
            Minus => match (left.clone(), right.clone()) {
                (Value::Number(l), Value::Number(r)) =>{
                    check_number_operands(&expr.operator, left, right)?;
                    Ok(Some(Value::Number(l - r)))
                },
                _ => Err(LoxErrors::RunTimeException(Error::new(expr.operator.line, "Operands must be two numbers or two strings.".to_string())))
            },
            Slash => match (left.clone(), right.clone()) {
                (Value::Number(l), Value::Number(r)) =>{
                    check_number_operands(&expr.operator, left, right)?;
                    Ok(Some(Value::Number(l / r)))
                } ,
                _ => Ok(None),
            },
            Star => match (left.clone(), right.clone()) {
                (Value::Number(l), Value::Number(r)) =>{
                    check_number_operands(&expr.operator, left, right)?;
                    Ok(Some(Value::Number(l * r)))
                } ,
                _ => Ok(None),
            },
            Plus => match (left.clone(), right.clone()) {
                (Value::Number(l), Value::Number(r)) =>{
                    check_number_operands(&expr.operator, left, right)?;
                    Ok(Some(Value::Number(l + r)))
                } ,
                (Value::String(l), Value::String(r)) => Ok(Some(Value::String(l + r.as_str()))),
                _ => Ok(None),
            },
            Greater => match(left.clone(), right.clone()) {
                (Value::Number(l), Value::Number(r)) => {
                    check_number_operands(&expr.operator, left, right)?;
                    Ok(Some(Value::Boolean(l > r)))
                },
                _ => Ok(None)
            },
            GreaterEqual => match(left.clone(), right.clone()) {
                (Value::Number(l), Value::Number(r)) =>{
                    check_number_operands(&expr.operator, left, right)?;
                    Ok(Some(Value::Boolean(l >= r)))
                },
                _ => Ok(None)
            },

            Less => match(left.clone(), right.clone()) {
                (Value::Number(l), Value::Number(r)) =>{
                    check_number_operands(&expr.operator, left, right)?;
                    Ok(Some(Value::Boolean(l < r)))
                } ,
                _ => Ok(None)

            },
            LessEqual => match(left.clone(), right.clone()) {
                (Value::Number(l), Value::Number(r)) =>{
                    check_number_operands(&expr.operator, left, right)?;
                    Ok(Some(Value::Boolean(l <= r)))
                } ,
                _ => Ok(None)
            },
            BangEqual => Ok(Some(Value::Boolean(!is_equal(&Some(left), &Some(right))))),
            Equal => Ok(Some(Value::Boolean(is_equal(&Some(left), &Some(right))))),
            _ => Err(LoxErrors::RunTimeException(Error::new(expr.operator.line, "Operands must be two numbers or two string".to_string())))
        }
    }
}
