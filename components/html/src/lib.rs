use error::HTMLError;
use std::{collections::HashMap, io::Write};
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
    pub fn new(ident: &str, attributes: Attributes) -> Result<Self, HTMLError> {
        Ok(HTMLElement {
            ident: ident.into(),
            attributes,
        })
    }

    pub fn open_tag(&self) -> String {
        format!("<{}{}>", self.ident, attributes_to_string(&self.attributes))
    }

    pub fn write_open_tag<W: Write>(&self, writer: &mut W) {
        writer.write_all(&self.open_tag().as_bytes()).unwrap();
    }

    pub fn close_tag(&self) -> String {
        format!("</{}>", self.ident)
    }

    pub fn write_close_tag<W: Write>(&self, writer: &mut W) {
        writer.write_all(&self.close_tag().as_bytes()).unwrap();
    }
}

fn attributes_to_string(attributes: &Attributes) -> String {
    // format attributes
    if !attributes.is_empty() {
        format!(
            " {}",
            attributes
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect::<Vec<String>>()
                .join(" ")
        )
    } else {
        String::new()
    }
}
