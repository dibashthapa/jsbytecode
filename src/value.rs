use std::cmp::Ordering;
use std::{
    fmt::{self, Debug},
    ops::{Add, Div, Mul, Sub},
};

use crate::error::{Error, LoxErrors};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
    ArithmeticError,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Nil => write!(f, "nil"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::ArithmeticError => write!(f, "Unable to evalute arithmetic expression"),
        }
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => Self::Number(left + right),
            (Self::String(left), Self::String(right)) => Self::String(format!("{left}{right}")),
            _ => Self::ArithmeticError,
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => Self::Number(left - right),
            _ => Self::ArithmeticError,
        }
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => Self::Number(left * right),
            _ => Self::ArithmeticError,
        }
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => Self::Number(left / right),
            _ => Self::ArithmeticError,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => left.partial_cmp(right),
            _ => None,
        }
    }
}

impl TryInto<f64> for Value {
    type Error = LoxErrors;
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::Number(number) => Ok(number),
            _ => Err(LoxErrors::RunTimeException(Error::new(
                0,
                "Cannot convert number to string".to_string(),
            ))),
        }
    }
}
