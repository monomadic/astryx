use crate::object::Object;
use error::AstryxResult;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type LocalData = HashMap<String, Object>;

#[derive(Clone, Default)]
pub struct State {
    pub local: LocalData,
    outer: Option<Rc<RefCell<State>>>,
}

impl<'a> State {
    // replace with default()
    pub fn new() -> Self {
        State {
            local: LocalData::new(),
            outer: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.local.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .outer
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone()),
        }
    }

    /// bind a variable to local state
    pub fn bind(&mut self, ident: &str, obj: Object) -> AstryxResult<()> {
        let _ = self.local.insert(ident.into(), obj.clone()); // return doesn't matter as all state is mutable
        Ok(()) // force return ok (this could change if mutability rules change, or overwriting builtins)
    }

    pub fn extend(outer: Rc<RefCell<Self>>) -> Self {
        Self {
            outer: Some(outer),
            ..Default::default()
        }
    }

    /// returns a flattened hashmap of all objects in state
    pub fn to_map(&self) -> HashMap<String, Object> {
        self.local.clone() // todo: inherit
    }
}
