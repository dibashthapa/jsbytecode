use std::collections::HashMap;

use crate::{
    error::{Error, LoxErrors, LoxResult},
    token::Token,
    value::Value,
};

pub struct Environment {
    values: HashMap<String, Option<Value>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}

impl Environment {
    pub fn define(&mut self, name: String, value: Option<Value>) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: Token) -> LoxResult<Option<Value>> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone());
        }
        Err(LoxErrors::RunTimeException(Error::new(
            name.line,
            format!("Undefined variable {} .", name.lexeme),
        )))
    }

    pub fn assign(&mut self, name: Token, value: Option<Value>) -> LoxResult<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }

        Err(LoxErrors::RunTimeException(Error::new(
            name.line,
            format!("Undefined variable {} .", name.lexeme),
        )))
    }
}
