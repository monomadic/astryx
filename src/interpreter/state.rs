use std::collections::HashMap;
use crate::{html::HTMLNode, imports::Imports, error::{AstryxError, AstryxResult}};
use rctree::Node;
use parser::variable::Variable;
use super::Value;

type LocalData = HashMap<String, Variable>;
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
    pub(crate) fn resolve(&self, variable: Variable) -> AstryxResult<Value> {
        Ok(match variable {
            Variable::QuotedString(s) => Value::String(s.clone()),
            Variable::RelativePath(p) => Value::String(p.clone()),
            _ => { return Err(AstryxError::new(&format!("cannot to_string: {:?}", self))); },
        })
    }
}
