use std::collections::HashMap;

use crate::{
    error::{Error, LoxErrors, LoxResult},
    token::Token,
    value::Value,
};

pub struct Environment {
    values: HashMap<String, Option<Value>>,
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
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}
