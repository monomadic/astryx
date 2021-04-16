use crate::object::Object;
use error::AstryxResult;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type LocalData = HashMap<String, Object>;

#[derive(Clone, Default)]
pub struct State {
    pub local: LocalData,
    outer: Option<Rc<RefCell<State>>>,
}

// // todo: duplicate. fix.
// fn span_to_location(span: Span) -> Location {
//     Location {
//         line: span.location_line(),
//         column: span.get_column(),
//         length: span.location_offset(),
//         filename: span.extra.into(),
//         context: String::from_utf8(span.get_line_beginning().into()).unwrap(),
//     }
// }

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

    // /// fetch a variable from state and throw an error upon failure
    // pub fn require(&self, ident: Span) -> AstryxResult<Object> {
    //     let i = ident.to_string();
    //     self.get(&i).ok_or(AstryxError::LocatedError(
    //         span_to_location(ident),
    //         AstryxErrorKind::MissingRequiredArgument(i),
    //     ))
    // }

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
