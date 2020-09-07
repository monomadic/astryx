use std::collections::HashMap;
use super::Value;
use crate::error::{AstryxError, AstryxResult};

pub(crate) type NamedArguments = HashMap<String, Value>;

pub(crate) trait TypeGetters {
    fn get_required_string(&self, ident: &str) -> AstryxResult<String>;
    fn get_string(&self, ident: &str) -> Option<String>;
}

impl TypeGetters for NamedArguments {
    fn get_string(&self, arg: &str) -> Option<String> {
        self
            .get(arg)
            .map(Value::to_string)
    }

    fn get_required_string(&self, ident: &str) -> AstryxResult<String> {
        self
            .get_string(ident)
            .ok_or(AstryxError::new(&format!("variable not found: {}", ident)))
    }
}
