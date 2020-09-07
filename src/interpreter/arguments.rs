use super::Value;
use crate::error::{AstryxError, AstryxErrorKind, AstryxResult};
use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) type NamedArguments = HashMap<String, Value>;

pub(crate) trait TypeGetters {
    fn get_required_string(&self, ident: &str) -> AstryxResult<String>;
    fn get_required_path(&self, ident: &str) -> AstryxResult<String>;
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

    fn get_required_string(&self, ident: &str) -> AstryxResult<String> {
        self.get_string(ident).ok_or(AstryxError::new_from(
            AstryxErrorKind::MissingRequiredArgument(String::from(ident)),
        ))
    }

    fn get_required_path(&self, ident: &str) -> AstryxResult<String> {
        println!("GET REQUIRED PATH {:?}", self.get_path(ident));
        self.get_path(ident).ok_or(AstryxError::new_from(
            AstryxErrorKind::MissingRequiredArgument(String::from(ident)),
        ))
    }
}
