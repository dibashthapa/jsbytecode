use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::{Error, LoxErrors, LoxResult},
    token::Token,
    value::Value,
};

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Option<Value>>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }
}

impl Environment {
    pub fn new(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

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
        if !self.values.contains_key(&name.lexeme) {
            if let Some(enclosing) = &mut self.enclosing {
                return enclosing.borrow_mut().assign(name, value);
            } else {
                return Err(LoxErrors::RunTimeException(Error::new(
                    name.line,
                    format!("Undefined variable {} .", name.lexeme),
                )));
            }
        }
        self.values.insert(name.lexeme, value);
        Ok(())
    }

}
