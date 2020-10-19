use std::collections::HashMap;
pub mod render;
pub mod program;

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
    pub fn new(ident: &str) -> Self {
        HTMLElement {
            ident: ident.into(),
            attributes: HashMap::new(),
        }
    }
}
