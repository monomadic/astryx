use std::collections::HashMap;
use crate::{html::HTMLNode, imports::Imports};
use rctree::Node;
use parser::variable::Variable;

type LocalData = HashMap<String, Variable>;
type Layouts = HashMap<String, Node<HTMLNode>>;

// TODO make private type
#[derive(Debug, Clone)]
pub(crate) struct State {
    local_variables: LocalData,
    pub(crate) pages: Layouts,
    imports: Imports,
}

impl State {
    pub fn new() -> Self {
        State {
            local_variables: LocalData::new(),
            pages: Layouts::new(),
            imports: Imports::new(),
        }
    }
}
