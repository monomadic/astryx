// writes an xml graph to a html string

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) struct HTMLNode {
    ident: String,
    attributes: HashMap<String, String>
}

impl HTMLNode {
    pub(crate) fn new() -> Self {
        Self {
            ident: "html".into(),
            attributes: HashMap::new(),
        }
    }
}
