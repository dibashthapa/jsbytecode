use std::fmt::{self, Debug};

use crate::error::{Error, LoxErrors};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Nil => write!(f, "nil"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
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
