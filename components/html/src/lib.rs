use std::collections::HashMap;
use error::HTMLError;
pub mod render;
// pub mod program;
pub mod error;

#[derive(Debug, Clone)]
pub enum HTMLNode {
    Element(HTMLElement),
    Text(String),
}

type Attributes = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct HTMLElement {
    ident: String,
    attributes: Attributes,
	// pub classes: Vec<String>,
	// pub styles: Vec<String>, // should be type safe
}

impl HTMLElement {
    pub fn new(ident: &str) -> Result<Self, HTMLError> {
        Ok(HTMLElement {
            ident: ident.into(),
            attributes: HashMap::new(),
        })
    }

    pub fn open_tag(&self) -> String {
        format!("<{}>", self.ident)
    }
}
