use crate::{
    models::Object, InterpreterError, InterpreterErrorKind, InterpreterResult,
};
use parser::Span;
use program::ProgramInstruction;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type LocalData = HashMap<String, Object>;

#[derive(Clone, Default)]
pub struct State {
    pub local: LocalData,
    outer: Option<Rc<RefCell<State>>>,
    program: Rc<RefCell<Vec<ProgramInstruction>>>,
    // pub writer: Writer,
}

impl<'a> State {
    // replace with default()
    pub fn new() -> Self {
        State {
            local: LocalData::new(),
            outer: None,
            program: Rc::new(RefCell::new(Vec::new())),
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

    /// fetch a variable from state and throw an error upon failure
    pub fn require(&self, name: &Span) -> InterpreterResult<Object> {
        self.get(&name.to_string()).ok_or(InterpreterError {
            kind: InterpreterErrorKind::InvalidReference(name.to_string()),
            location: Some((*name).into()),
        })
    }

    /// bind a variable to local state
    pub fn bind(&mut self, ident: &str, obj: Object) -> InterpreterResult<()> {
        let _ = self.local.insert(ident.into(), obj.clone()); // return doesn't matter as all state is mutable
        Ok(()) // force return ok (this could change if mutability rules change, or overwriting builtins)
    }

    pub fn push_instruction(&self, instruction: ProgramInstruction) {
        self.program.borrow_mut().push(instruction)
    }

    pub fn get_program(&self) -> Vec<ProgramInstruction> {
        self.program.borrow().clone()
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
