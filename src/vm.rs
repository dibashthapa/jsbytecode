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
use std::collections::HashMap;
type Literal = Option<Value>;

#[derive(Clone)]
pub enum ByteCode {
    Return,
    NewString(Registers, String),
    Add(Registers, Registers, Registers),
    Mul(Registers, Registers, Registers),
    Sub(Registers, Registers, Registers),
    Div(Registers, Registers, Registers),
    SetVariable(String, Registers),
    GetVariable(String, Registers),
    Load(Registers, Value),
}

impl std::fmt::Debug for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Return => write!(f, "Return"),
            Self::Load(reg, value) => write!(f, "Load {:?} {}", reg, value),
            Self::Add(dst, src1, src2) => {
                write!(f, "Add {:?}, {:?}, {:?}", dst, src1, src2)
            }
            Self::Mul(dst, src1, src2) => write!(f, "Mul {:?}, {:?}, {:?}", dst, src1, src2),
            Self::Sub(dst, src1, src2) => write!(f, "Sub {:?}, {:?}, {:?}", dst, src1, src2),
            Self::Div(dst, src1, src2) => write!(f, "Div {:?}, {:?}, {:?}", dst, src1, src2),
            Self::NewString(dst, src) => write!(f, "NewString {:?} {}", dst, src),
            Self::SetVariable(name, reg) => write!(f, "SetVariable {} {:?}", name, reg),
            Self::GetVariable(name, reg) => write!(f, "GetVariable {:?} {:?}", reg, name),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum Registers {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
}

impl From<Registers> for usize {
    fn from(reg: Registers) -> usize {
        match reg {
            Registers::R1 => 1,
            Registers::R2 => 2,
            Registers::R3 => 3,
            Registers::R4 => 4,
            Registers::R5 => 5,
            Registers::R6 => 6,
            Registers::R7 => 7,
            Registers::R8 => 8,
            Registers::R9 => 9,
            Registers::R10 => 10,
            Registers::R11 => 11,
            Registers::R12 => 12,
        }
    }
}

impl Into<Registers> for usize {
    fn into(self) -> Registers {
        match self {
            1 => Registers::R1,
            2 => Registers::R2,
            3 => Registers::R3,
            4 => Registers::R4,
            5 => Registers::R5,
            6 => Registers::R6,
            7 => Registers::R7,
            8 => Registers::R8,
            9 => Registers::R9,
            10 => Registers::R10,
            11 => Registers::R11,
            12 => Registers::R12,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone)]
pub struct Vm {
    pub registers: HashMap<Registers, Value>,
    pc: usize,
    stack: Vec<f64>,
    program: Vec<ByteCode>,
    variables: HashMap<String, Value>,
}

impl Vm {
    pub fn new(bytecodes: Vec<ByteCode>) -> Self {
        Self {
            registers: HashMap::new(),
            pc: 0,
            stack: Vec::new(),
            variables: HashMap::new(),
            program: bytecodes,
        }
    }

    fn read_byte(&mut self) -> ByteCode {
        let byte = self.program.get(self.pc).unwrap_or(&ByteCode::Return);
        self.pc += 1;
        byte.clone()
    }

    fn read_register(&mut self, register: Registers) -> Value {
        self.registers.get(&register).unwrap().clone()
    }

    fn write_register(&mut self, register: Registers, value: Value) {
        self.registers.insert(register, value);
    }

    pub fn interpret(&mut self) {
        loop {
            let instruction = self.read_byte();
            match instruction {
                ByteCode::Return => break,
                ByteCode::Load(dst, value) => {
                    self.write_register(dst, value);
                }
                ByteCode::Add(dst, src1, src2) => {
                    let result = self.read_register(src1) + self.read_register(src2);
                    self.write_register(dst, result);
                }
                ByteCode::Mul(dst, src1, src2) => {
                    let result = self.read_register(src1) * self.read_register(src2);
                    self.write_register(dst, result);
                }
                ByteCode::Sub(dst, src1, src2) => {
                    let result = self.read_register(src1) - self.read_register(src2);
                    self.write_register(dst, result);
                }
                ByteCode::Div(dst, src1, src2) => {
                    let result = self.read_register(src1) / self.read_register(src2);
                    self.write_register(dst, result);
                }
                ByteCode::SetVariable(src, register) => {
                    let value = self.read_register(register);
                    self.variables.insert(src, value);
                }
                ByteCode::GetVariable(dest, register) => {
                    let value = self.variables.get(&dest);
                    if value.is_some() {
                        self.write_register(register, value.unwrap().clone());
                    }
                }
                _ => break,
            }
        }
    }
}

#[derive(Default)]
pub struct ByteCodeGenerator {
    pub bytecodes: Vec<ByteCode>,
    register_count: usize,
    variables: HashMap<String, Registers>,
    values: HashMap<String, Value>,
}

impl ByteCodeGenerator {
    fn evaluate(&mut self, expr: &Expr) -> LoxResult<Literal> {
        expr.accept(self)
    }

    pub fn intrepret(&mut self, statements: &[Stmt]) -> LoxResult<()> {
        for stmt in statements.iter() {
            self.execute(stmt)?
        }
        Ok(())
    }

    fn current_register(&self) -> Registers {
        self.register_count.into()
    }

    fn execute(&mut self, stmt: &Stmt) -> LoxResult<()> {
        stmt.accept(self)
    }

    fn emit_bytecode(&mut self, bytecode: ByteCode) {
        self.bytecodes.push(bytecode);
    }

    fn allocate_register(&mut self) -> usize {
        self.register_count += 1;
        self.register_count
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
        dbg!(&self.variables);
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
            src1 = self.variables.get(&variable.name.lexeme).unwrap().to_owned();
        }
        let right = self.evaluate(&expr.right)?.unwrap();
        let mut src2 = self.current_register();
        let dest = self.allocate_register();
        if let Expr::Variable(variable) = expr.right.as_ref() {
            src2 = self.variables.get(&variable.name.lexeme).unwrap().to_owned();
        }

        match expr.operator.type_ {
            Minus => {
                self.emit_bytecode(ByteCode::Sub(dest.into(), src1, src2));
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left - right))
            }
            Slash => {
                self.emit_bytecode(ByteCode::Div(dest.into(), src1, src2));
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left / right))
            }
            Star => {
                self.emit_bytecode(ByteCode::Mul(dest.into(), src1, src2));
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Some(left * right))
            }
            Plus => {
                self.emit_bytecode(ByteCode::Add(dest.into(), src1, src2));
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
        let value = self.evaluate(&stmt.expression)?.unwrap();
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

        self.values.insert(identifier, value.unwrap());
        self.emit_bytecode(ByteCode::SetVariable(
            stmt.name.lexeme.clone(),
            self.current_register(),
        ));
        // let value = self.evaluate(&stmt.initializer)?;
        // self.environment
        //     .borrow_mut()
        //     .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Self::Result {
        // self.execute_block(&stmt.statements, self.environment.clone(), self.repl)?;
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Self::Result {
        while is_truthy(&self.evaluate(&stmt.condition)?.unwrap()) {
            self.execute(&stmt.body)?;
        }
        Ok(())
    }
}
