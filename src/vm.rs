use crate::generator::ByteCode;
use crate::value::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Label {
    pub name: String,
    pub address: usize,
}

impl Label {
    pub fn new(name: String, address: usize) -> Self {
        Self { name, address }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_address(&self) -> usize {
        self.address
    }
}

#[derive(Clone, Default)]
struct Flags {
    pub less_than: bool,
    pub greater_than: bool,
    pub equal: bool,
}

impl Flags {
    pub fn set_less_than(&mut self, value: bool) {
        self.less_than = value;
    }

    pub fn set_greater_than(&mut self, value: bool) {
        self.greater_than = value;
    }

    pub fn set_equal(&mut self, value: bool) {
        self.equal = value;
    }
}

#[derive(Clone)]
pub struct Vm {
    pub registers: HashMap<i32, Value>,
    pc: usize,
    flag: bool,
    stack: Vec<f64>,
    program: Vec<ByteCode>,
    labels: HashMap<String, i32>,
    variables: HashMap<String, Value>,
}

impl Vm {
    pub fn new(bytecodes: Vec<ByteCode>) -> Self {
        Self {
            registers: HashMap::new(),
            pc: 0,
            flag: false,
            labels: HashMap::new(),
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

    fn read_register(&mut self, register: i32) -> Value {
        self.registers.get(&register).unwrap().clone()
    }

    fn write_register(&mut self, register: i32, value: Value) {
        self.registers.insert(register, value);
    }

    fn write_label(&mut self, label: String) {
        self.labels.insert(label, self.pc as i32);
    }

    fn get_label(&self, label: &str) -> i32 {
        *self.labels.get(label).unwrap()
    }

    pub fn interpret(&mut self) {
        loop {
            let instruction = self.read_byte();
            match instruction {
                ByteCode::Return => break,
                ByteCode::LoadUndefined(reg) => {
                    self.write_register(reg, Value::Undefined);
                }
                ByteCode::Load(dst, value) => {
                    self.write_register(dst, value);
                }
                ByteCode::Label(label) => {
                    self.write_label(label);
                }
                ByteCode::JumpIfTrue(label) => {
                    if self.flag {
                        self.pc = self.get_label(&label) as usize;
                    }
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
                ByteCode::Print(register) => {
                    let value = self.read_register(register);
                    println!("{}", value);
                }
                ByteCode::TestLessThan(lhs, rhs) => {
                    let result = self.read_register(lhs) < self.read_register(rhs);
                    self.flag = result;
                }
                ByteCode::TestLessEqThan(lhs, rhs) => {
                    let result = self.read_register(lhs) <= self.read_register(rhs);
                    self.flag = result;
                }
                ByteCode::TestGreaterThan(lhs, rhs) => {
                    let result = self.read_register(lhs) > self.read_register(rhs);
                    self.flag = result;
                }
                ByteCode::TestGreaterEqThan(lhs, rhs) => {
                    let result = self.read_register(lhs) >= self.read_register(rhs);
                    self.flag = result;
                }
                _ => {
                    break;
                }
            }
        }
    }
}
