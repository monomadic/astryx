use std::collections::HashMap;
use crate::{models::Value, InterpreterResult, InterpreterError};
use parser::Expression;

type LocalData = HashMap<String, Value>;

#[derive(Debug)]
pub struct State {
    locals: LocalData,
    // pub(crate) pages: Layouts,
    // pub(crate) imports: Imports,
    // pub(crate) pwd: String,
}

impl State {
    pub fn new() -> Self {
        State {
            locals: LocalData::new(),
        }
    }

    pub fn bind(&mut self, ident: &str, value: Value) -> InterpreterResult<()> {
        match self.locals.insert(ident.into(), value) {
            Some(_) => Ok(()),
            None => Err(InterpreterError::Unhandled)
        }
    }

    pub fn eval(&self, expr: &Expression) -> InterpreterResult<Value> {
        // match expr {
        //     Expression::FunctionCall(_) => {}
        //     Expression::Reference(_) => {}
        //     Expression::Literal(v) => {}
        // }
        Ok(Value::String("stringy".into()))
    }
}
