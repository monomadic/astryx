use error::HTMLError;
use std::collections::HashMap;
pub mod error;
pub mod render;

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

    // pub fn write_open_tag<W: Write>(&self, writer: &mut W) {
    //     writer
    //         .write_fmt(format_args!("{}", &self.open_tag()))
    //         .unwrap();
    // }

    pub fn close_tag(&self) -> String {
        format!("</{}>", self.ident)
    }

    // pub fn write_close_tag<W: Write>(&self, writer: &mut W) {
    //     // let mut buf = BufWriter::new(writer);

    //     writer
    //         .write_fmt(format_args!("{}", &self.close_tag()))
    //         .unwrap();
    // }
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
