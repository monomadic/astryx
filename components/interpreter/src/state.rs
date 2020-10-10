use std::collections::HashMap;
use crate::models::Value;

type LocalData = HashMap<String, Value>;

#[derive(Debug)]
pub(crate) struct State {
    locals: LocalData,
    // pub(crate) pages: Layouts,
    // pub(crate) imports: Imports,
    // pub(crate) pwd: String,
}

impl State {
    pub(crate) fn new() -> Self {
        State {
            locals: LocalData::new(),
        }
    }
}
