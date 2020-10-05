use super::Value;
use std::collections::HashMap;
use crate::error::{InterpreterError, InterpreterResult};

pub(crate) type NamedArguments = HashMap<String, Value>;

pub(crate) trait TypeGetters {
    fn get_required_string(&self, ident: &str) -> InterpreterResult<String>;
    fn get_required_path(&self, ident: &str) -> InterpreterResult<String>;
    fn get_string(&self, ident: &str) -> Option<String>;
    fn get_path(&self, ident: &str) -> Option<String>;
}

impl TypeGetters for NamedArguments {
    fn get_string(&self, arg: &str) -> Option<String> {
        self.get(arg).map(Value::to_string)
    }

    fn get_path(&self, arg: &str) -> Option<String> {
        println!("GET PATH: {:?}", self.get(arg));
        self.get(arg).and_then(|v| match v {
            Value::Path(p) => Some(p.clone()),
            _ => None,
        })
    }

    fn get_required_string(&self, ident: &str) -> InterpreterResult<String> {
        self.get_string(ident).ok_or(InterpreterError::MissingRequiredArgument)
    }

    fn get_required_path(&self, ident: &str) -> InterpreterResult<String> {
        println!("GET REQUIRED PATH {:?}", self.get_path(ident));
        self.get_path(ident).ok_or(InterpreterError::MissingRequiredArgument)
    }
}
