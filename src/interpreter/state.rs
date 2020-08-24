use super::Value;
use crate::{
    error::{AstryxError, AstryxResult},
    html::HTMLNode,
    imports::Imports,
};
use parser::{parser::StringToken, variable::Variable};
use rctree::Node;
use std::collections::HashMap;

type LocalData = HashMap<String, Value>;
type Layouts = HashMap<String, Node<HTMLNode>>;

// TODO make private type
#[derive(Debug, Clone)]
pub(crate) struct State {
    pub(crate) local_variables: LocalData,
    pub(crate) pages: Layouts,
    pub(crate) imports: Imports,
}

impl State {
    pub(crate) fn new() -> Self {
        State {
            local_variables: LocalData::new(),
            pages: Layouts::new(),
            imports: Imports::new(),
        }
    }

    /// use local state to resolve variables into static constants at runtime.
    pub(crate) fn resolve(&self, variable: &Variable) -> AstryxResult<Value> {
        Ok(match variable {
            Variable::QuotedString(s) => Value::String(s.clone()),
            Variable::Reference(r) => {
                self
                    .get(r)
                    .map(|v| v.clone())
                    .ok_or(AstryxError::new(format!("no such variable in scope: {}. locals: {:#?}", r, self.local_variables)))?
            }
            Variable::RelativePath(p) => Value::Documents(crate::filesystem::read_documents(&p)?),
            _ => {
                return Err(AstryxError::new(&format!("cannot to_string: {:?}", variable)));
            }
        })
    }

    pub(crate) fn insert<S:ToString>(&mut self, ident: S, value: &Value) {
        self.local_variables.insert(ident.to_string(), value.clone());
    }

    pub(crate) fn get<S:ToString>(&self, ident: S) -> Option<&Value> {
        let segments = ident.to_string();
        let segments = segments.split(".").collect::<Vec<&str>>();

        match segments.len() {
            0 => self.local_variables.get(&ident.to_string()),
            1 => None,
            _ => {
                self.local_variables.get(&segments[0].to_string())
            },
        }
        
    }

    pub(crate) fn interpolate_string(&self, tokens: &Vec<StringToken>) -> AstryxResult<String> {
        tokens
            .iter()
            .map(|token| match token {
                StringToken::Text(s) => Ok(s.clone()),
                StringToken::Variable(v) => self.resolve(v).map(|v| v.to_string()),
            })
            .collect()
    }
}
