use std::collections::HashMap;

use crate::ast::{
    Assign, Binary, BlockStmt, Expr, ExpressionStmt, Grouping, IfStmt, Literal as LiteralExp,
    Logical, PrintStmt, Stmt, Unary, VarStmt, Variable, VisitorExpr, VisitorStmt, WhileStmt,
};
use crate::error::{Error, LoxErrors, LoxResult};
use crate::token::Token;

use crate::token_type::TokenType::{
    self, Bang, BangEqual, EqualEqual, Greater, GreaterEqual, Less, LessEqual, Minus, Plus, Slash,
    Star,
};
use crate::tools::*;

use crate::value::Value;
type Literal = Option<Value>;

#[derive(Clone)]
pub enum ByteCode {
    Return,
    LoadUndefined(i32),
    NewString(i32, String),
    Add(i32, i32, i32),
    Mul(i32, i32, i32),
    Sub(i32, i32, i32),
    Div(i32, i32, i32),
    SetVariable(String, i32),
    GetVariable(String, i32),
    Load(i32, Value),
    JumpIfTrue(String),
    TestLessThan(i32, i32),
    TestGreaterThan(i32, i32),
    TestLessEqThan(i32, i32),
    TestGreaterEqThan(i32, i32),
    Label(String),
    Print(i32),
}

impl std::fmt::Debug for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Return => write!(f, "Return"),
            Self::LoadUndefined(reg) => write!(f, "LoadUndefined R{:?}", reg),
            Self::Load(reg, value) => write!(f, "Load R{:?} {}", reg, value),
            Self::Add(dst, src1, src2) => {
                write!(f, "Add R{:?}, R{:?}, R{:?}", dst, src1, src2)
            }
            Self::Mul(dst, src1, src2) => write!(f, "Mul Reg {} {:?}, {:?}", dst, src1, src2),
            Self::Sub(dst, src1, src2) => write!(f, "Sub Reg {} {:?}, {:?}", dst, src1, src2),
            Self::Div(dst, src1, src2) => write!(f, "Div Reg {} {:?}, {:?}", dst, src1, src2),
            Self::NewString(dst, src) => write!(f, "NewString Reg {} {}", dst, src),
            Self::SetVariable(name, reg) => write!(f, "SetVariable {} R{:?}", name, reg),
            Self::GetVariable(name, reg) => write!(f, "GetVariable R{} {:?}", reg, name),
            Self::JumpIfTrue(label) => write!(f, "JumpIfTrue {:?}", label),
            Self::TestLessThan(src1, src2) => write!(f, "TestLessThan R{:?}, R{:?}", src1, src2),
            Self::TestGreaterThan(src1, src2) => {
                write!(f, "TestGreaterThan R{:?}, R{:?}", src1, src2)
            }
            Self::TestLessEqThan(src1, src2) => {
                write!(f, "TestLessEqThan R{:?}, R{:?}", src1, src2)
            }
            Self::TestGreaterEqThan(src1, src2) => {
                write!(f, "TestGreaterEqThan R{:?}, R{:?}", src1, src2)
            }
            Self::Label(label) => write!(f, "{}:", label),
            Self::Print(value) => write!(f, "Print R{:?}", value),
        }
    }
}

#[derive(Default)]
pub struct ByteCodeGenerator {
    pub bytecodes: Vec<ByteCode>,
    register_count: usize,
    label_count: usize,
    variables: HashMap<String, i32>,
    values: HashMap<String, Value>,
}

impl ByteCodeGenerator {
    fn evaluate(&mut self, expr: &Expr) -> LoxResult<Literal> {
        expr.accept(self)
    }

    fn allocate_label(&mut self) -> String {
        self.label_count += 1;
        format!("L{}", self.label_count)
    }

    pub fn intrepret(&mut self, statements: &[Stmt]) -> LoxResult<()> {
        for stmt in statements.iter() {
            self.execute(stmt)?
        }
        Ok(())
    }

    fn execute_block(&mut self, statements: &Vec<Stmt>) -> LoxResult<()> {
        statements.iter().try_for_each(|stmt| self.execute(stmt))?;
        Ok(())
    }

    fn current_register(&self) -> i32 {
        self.register_count as i32
    }

    fn execute(&mut self, stmt: &Stmt) -> LoxResult<()> {
        stmt.accept(self)
    }

    fn emit_bytecode(&mut self, bytecode: ByteCode) {
        self.bytecodes.push(bytecode);
    }

    fn allocate_register(&mut self) -> i32 {
        self.register_count += 1;
        self.register_count as i32
    }
}

impl VisitorExpr for ByteCodeGenerator {
    type Result = LoxResult<Literal>;

    fn visit_logical_expr(&mut self, expr: &Logical) -> Self::Result {
        let left = self.evaluate(&expr.left)?;

        if let Some(left) = left {
            if expr.operator.type_ == TokenType::Or && is_truthy(&left) {
                return Ok(Some(left));
            }
        }

        self.evaluate(&expr.right)
    }

    fn visit_assign_expr(&mut self, expr: &Assign) -> Self::Result {
        let value = self.evaluate(&expr.value)?;

        if expr.name.is_identifier() {
            let identifier = expr.name.clone();
            let reg = self.current_register();
            self.variables.insert(identifier.lexeme.clone(), reg);
            self.emit_bytecode(ByteCode::SetVariable(identifier.lexeme, reg));
        }
        self.values
            .insert(expr.name.lexeme.to_string(), value.clone().unwrap());
        Ok(value)
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> Self::Result {
        let identifier = expr.name.clone();
        let register = self.variables.get(&identifier.lexeme).unwrap();
        self.emit_bytecode(ByteCode::GetVariable(identifier.lexeme.clone(), *register));
        let value = self.values.get(&identifier.lexeme).cloned();
        Ok(value)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExp) -> Self::Result {
        let dest = self.allocate_register();
        let value = expr.value.to_owned();

        match value.clone() {
            Some(Value::Number(_)) => {
                self.emit_bytecode(ByteCode::Load(dest.into(), value.clone().unwrap()))
            }
            Some(Value::String(string)) => {
                self.emit_bytecode(ByteCode::NewString(dest.into(), string))
            }
            _ => {}
        }

        Ok(value)
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Self::Result {
        self.evaluate(&expr.expression)
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Self::Result {
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
    fn visit_binary_exp(&mut self, expr: &Binary) -> Self::Result {
        let left = self.evaluate(&expr.left)?.unwrap();
        let mut src1 = self.current_register();
        if let Expr::Variable(variable) = expr.left.as_ref() {
            src1 = self
                .variables
                .get(&variable.name.lexeme)
                .unwrap()
                .to_owned();
        }
        let right = self.evaluate(&expr.right)?.unwrap();
        let mut src2 = self.current_register();
        let dest = self.allocate_register();
        if let Expr::Variable(variable) = expr.right.as_ref() {
            src2 = self
                .variables
                .get(&variable.name.lexeme)
                .unwrap()
                .to_owned();
        }

        match expr.operator.type_ {
            Minus => {
                self.emit_bytecode(ByteCode::Sub(dest, src1, src2));
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left - right))
            }
            Slash => {
                self.emit_bytecode(ByteCode::Div(dest, src1, src2));
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left / right))
            }
            Star => {
                self.emit_bytecode(ByteCode::Mul(dest, src1, src2));
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left * right))
            }
            Plus => {
                self.emit_bytecode(ByteCode::Add(dest, src1, src2));
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left + right))
            }
            Greater => {
                self.emit_bytecode(ByteCode::TestGreaterThan(src1, src2));
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(Value::Boolean(left > right)))
            }
            GreaterEqual => {
                self.emit_bytecode(ByteCode::TestGreaterEqThan(src1, src2));
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(Value::Boolean(left >= right)))
            }
            Less => {
                self.emit_bytecode(ByteCode::TestLessThan(src1, src2));
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(Value::Boolean(left < right)))
            }
            LessEqual => {
                self.emit_bytecode(ByteCode::TestLessEqThan(src1, src2));
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

impl VisitorStmt for ByteCodeGenerator {
    type Result = LoxResult<()>;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Self::Result {
        let value = self.evaluate(&stmt.condition)?.unwrap();
        if is_truthy(&value) {
            self.execute(&stmt.then_branch)?;
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch.as_ref())?;
        }

        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Self::Result {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Self::Result {
        let expr = stmt.expression.clone();
        if let Expr::Grouping(expr) = expr {
            if let Expr::Variable(variable) = *expr.expression.clone() {
                let register = self.variables.get(&variable.name.lexeme).unwrap();
                self.emit_bytecode(ByteCode::Print(register.clone()));
            }
        }
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Self::Result {
        let mut value = None;
        if let Some(initializer) = &stmt.initializer {
            value = self.evaluate(initializer)?;
        }
        let identifier = stmt.name.lexeme.clone();

        self.variables
            .insert(identifier.clone(), self.current_register());

        if let Some(value) = value {
            self.values.insert(identifier, value);
        } else {
            let reg = self.allocate_register();
            self.emit_bytecode(ByteCode::LoadUndefined(reg));
            self.values.insert(identifier, Value::Undefined);
        }
        self.emit_bytecode(ByteCode::SetVariable(
            stmt.name.lexeme.clone(),
            self.current_register(),
        ));
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Self::Result {
        self.execute_block(&stmt.statements)?;
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Self::Result {
        let label = self.allocate_label();

        let body = *stmt.body.clone();
        self.emit_bytecode(ByteCode::Label(label.clone()));

        match body {
            Stmt::BlockStmt(block) => block.accept(self)?,
            _ => {
                self.execute(&body)?;
            }
        }
        stmt.condition.accept(self)?;
        self.emit_bytecode(ByteCode::JumpIfTrue(label));

        /*
        LOAD R1, 5
        LOAD R2, 10
        {label}:
            PRINT R1
            INCR R1, 1
            TestLessThan R1, R2
            JmpIfTrue {label}
        */
        Ok(())
    }
}

fn is_truthy(object: &Value) -> bool {
    match *object {
        Value::Nil => false,
        Value::Boolean(b) => b,
        _ => false,
    }
}

fn is_equal(a: &Literal, b: &Literal) -> bool {
    if a.is_none() && b.is_none() {
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
