use std::collections::HashMap;

use crate::ast::{Expression, Statement};

use crate::value::Value;
type Literal = Option<Value>;

trait CompilerEvaluation {
    fn compile(&mut self, ctx: &mut ByteCodeGenerator);
}

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

impl CompilerEvaluation for Statement {
    fn compile(&mut self, ctx: &mut ByteCodeGenerator) {
        match self {
            Self::If {
                condition,
                then_branch,
                else_branch,
            } => {
                condition.compile(ctx);
                then_branch.compile(ctx);
            }
            Self::Print { expression } => {
                if let Expression::Grouping { expression } = expression {
                    if let Expression::Variable { name } = expression.as_ref() {
                        if let Some(register) = ctx.variables.get(&name.lexeme) {
                            ctx.emit_bytecode(ByteCode::Print(*register));
                        }
                    }
                }
            }
            Self::Var { name, initializer } => {
                if let Some(init) = initializer {
                    init.compile(ctx);
                }

                let ident = &name.lexeme;

                ctx.variables.insert(ident.clone(), ctx.current_register());

                let reg = ctx.allocate_register();
                ctx.emit_bytecode(ByteCode::LoadUndefined(reg));
                ctx.values.insert(ident.clone(), Value::Undefined);
                ctx.emit_bytecode(ByteCode::SetVariable(
                    ident.to_owned(),
                    ctx.current_register(),
                ));
            }
            Self::Block { statements } => {
                for stmt in statements {
                    stmt.compile(ctx);
                }
            }
            Self::While { condition, body } => {
                let label = ctx.allocate_label();
                body.compile(ctx);
                condition.compile(ctx);
                ctx.emit_bytecode(ByteCode::JumpIfTrue(label));
            }
            Self::Expression { expression } => {
                expression.compile(ctx);
            }
        }
    }
}

impl CompilerEvaluation for Expression {
    fn compile(&mut self, ctx: &mut ByteCodeGenerator) {
        match self {
            Self::Literal { value } => {
                let dest = ctx.allocate_register();
                if let Some(value) = value {
                    match value {
                        Value::Number(num) => {
                            ctx.emit_bytecode(ByteCode::Load(dest.into(), Value::Number(*num)))
                        }
                        Value::String(ref string) => {
                            ctx.emit_bytecode(ByteCode::NewString(dest.into(), string.clone()))
                        }
                        _ => {}
                    }
                }
            }

            Self::Logical {
                left,
                operator,
                right,
            } => {
                left.compile(ctx);
                right.compile(ctx);
            }
            Self::Binary {
                left,
                operator,
                right,
            } => {
                left.compile(ctx);
                let src1 = ctx.current_register();
                right.compile(ctx);
                let src2 = ctx.current_register();

                let dest = ctx.allocate_register();

                match operator.type_ {
                    crate::token_type::TokenType::Minus => {
                        ctx.emit_bytecode(ByteCode::Sub(dest, src1, src2))
                    }
                    crate::token_type::TokenType::Slash => {
                        ctx.emit_bytecode(ByteCode::Div(dest, src1, src2))
                    }
                    crate::token_type::TokenType::Star => {
                        ctx.emit_bytecode(ByteCode::Mul(dest, src1, src2))
                    }
                    crate::token_type::TokenType::Plus => {
                        ctx.emit_bytecode(ByteCode::Add(dest, src1, src2));
                    }
                    crate::token_type::TokenType::Greater => {
                        ctx.emit_bytecode(ByteCode::TestGreaterThan(src1, src2));
                    }
                    _ => {}
                }
            }
            Self::Unary { operator, right } => {
                right.compile(ctx);
            }
            Self::Grouping { expression } => {
                expression.compile(ctx);
            }
            Self::Variable { name } => {
                let register = ctx.variables.get(&name.lexeme).unwrap();
                ctx.emit_bytecode(ByteCode::GetVariable(name.lexeme.clone(), *register));
            }
            Self::Assign { name, value } => {
                value.compile(ctx);

                if name.is_identifier() {
                    let reg = ctx.current_register();
                    ctx.variables.insert(name.lexeme.clone(), reg);
                }
            }
        }
    }
}

impl ByteCodeGenerator {
    fn allocate_label(&mut self) -> String {
        self.label_count += 1;
        format!("L{}", self.label_count)
    }

    fn current_register(&self) -> i32 {
        self.register_count as i32
    }

    fn emit_bytecode(&mut self, bytecode: ByteCode) {
        self.bytecodes.push(bytecode);
    }

    fn allocate_register(&mut self) -> i32 {
        self.register_count += 1;
        self.register_count as i32
    }
}
//
//
// /*
//      program        → statement* EOF ;
//      statement      → exprStmt
//                     | printStmt ;
//      exprStmt       → expression ";" ;
//      printStmt      → "print" expression ";" ;
// */
//
// impl<'a> VisitorStmt<'a> for ByteCodeGenerator {
//     fn visit_variable_stmt(&mut self, stmt: &Box<VariableStatement<'a>>) {}
//
//     fn visit_if_stmt(&mut self, stmt: &Box<IfStatement<'a>>) {}
//
//     fn visit_block_stmt(&mut self, stmt: &Box<BlockStatement<'a>>) {}
//
//     fn visit_print_stmt(&mut self, stmt: &Box<PrintStatement<'a>>) {}
//
//     fn visit_while_stmt(&mut self, stmt: &Box<WhileStatement<'a>>) {}
//
//     fn visit_expression_stmt(&mut self, stmt: &Box<ExpressionStatement<'a>>) {}
//     //
//     // fn visit_if_stmt(&mut self, stmt: &IfStatement) {
//     //     let value = self.evaluate(&stmt.condition)?.unwrap();
//     //     if is_truthy(&value) {
//     //         self.execute(&stmt.then_branch)?;
//     //     } else if let Some(else_branch) = &stmt.else_branch {
//     //         self.execute(else_branch.as_ref())?;
//     //     }
//     // }
//     //
//     // fn visit_expression_stmt(&mut self, stmt: &ExpressionStatement) {
//     //     self.evaluate(&stmt.expression)?;
//     //     Ok(())
//     // }
//     //
//     // fn visit_print_stmt(&mut self, stmt: &PrintStatement) {
//     //     let expr = stmt.expression.clone();
//     //     if let Expression::Grouping(expr) = expr {
//     //         if let Expression::Variable(variable) = *expr.expression.clone() {
//     //             let register = self.variables.get(&variable.name.lexeme).unwrap();
//     //             self.emit_bytecode(ByteCode::Print(register.clone()));
//     //         }
//     //     }
//     //     Ok(())
//     // }
//     //
//     // fn visit_variable_stmt(&mut self, stmt: &Box<VariableStatement<'a>>) {
//     //     let mut value = None;
//     //     if let Some(initializer) = &stmt.initializer {
//     //         value = self.evaluate(initializer)?;
//     //     }
//     //     let identifier = stmt.name.lexeme.clone();
//     //
//     //     self.variables
//     //         .insert(identifier.clone(), self.current_register());
//     //
//     //     if let Some(value) = value {
//     //         self.values.insert(identifier, value);
//     //     } else {
//     //         let reg = self.allocate_register();
//     //         self.emit_bytecode(ByteCode::LoadUndefined(reg));
//     //         self.values.insert(identifier, Value::Undefined);
//     //     }
//     //     self.emit_bytecode(ByteCode::SetVariable(
//     //         stmt.name.lexeme.clone(),
//     //         self.current_register(),
//     //     ));
//     // }
//     //
//     // fn visit_block_stmt(&mut self, stmt: &BlockStatement) {
//     //     self.execute_block(&stmt.statements)?;
//     // }
//     //
//     // fn visit_while_stmt(&mut self, stmt: &WhileStatement) {
//     //     let label = self.allocate_label();
//     //
//     //     let body = *stmt.body.clone();
//     //     self.emit_bytecode(ByteCode::Label(label.clone()));
//     //
//     //     match body {
//     //         Statement::BlockStmt(block) => block.accept(self)?,
//     //         _ => {
//     //             self.execute(&body)?;
//     //         }
//     //     }
//     //     stmt.condition.accept(self)?;
//     //     self.emit_bytecode(ByteCode::JumpIfTrue(label));
//     //
//     //     /*
//     //     {label}:
//     //         PRINT R1
//     //         INCR R1, 1
//     //         TestLessThan R1, R2
//     //         JmpIfTrue {label}
//     //     */
//     //     Ok(())
//     // }
// }
//
fn is_truthy(object: &Value) -> bool {
    match *object {
        Value::Nil => false,
        Value::Boolean(b) => b,
        _ => false,
    }
}
//
// fn is_equal(a: &Literal, b: &Literal) -> bool {
//     if a.is_none() && b.is_none() {
//         false
//     } else {
//         a.eq(b)
//     }
// }
//
// fn check_number_and_operand(operator: &Token, operand: &Value) -> LoxResult<()> {
//     match operand {
//         Value::Number(_) => Ok(()),
//         _ => Err(LoxErrors::RunTimeException(Error::new(
//             operator.line,
//             "Operand must be a number".to_string(),
//         ))),
//     }
// }
//
// fn check_number_operands(operator: &Token, left: &Value, right: &Value) -> LoxResult<()> {
//     match (left, right) {
//         (Value::Number(_), Value::Number(_)) => Ok(()),
//         _ => Err(LoxErrors::RunTimeException(Error::new(
//             operator.line,
//             "Operands must be numbers".to_string(),
//         ))),
//     }
// }
