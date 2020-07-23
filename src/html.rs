// writes an xml graph to a html string

use crate::error::{AstryxError, AstryxResult};
use rctree::Node;
use std::{collections::HashMap, fmt::Write};

#[derive(Debug, Clone)]
pub(crate) enum HTMLNode {
    Element(HTMLElement),
    Text(String),
}

#[derive(Debug, Clone)]
pub(crate) struct HTMLElement {
    ident: String,
    attributes: HashMap<String, String>,
}

impl HTMLNode {
    pub(crate) fn new(ident: &str) -> Self {
        HTMLNode::Element(HTMLElement {
            ident: ident.into(),
            attributes: HashMap::new(),
        })
    }
}

pub(crate) fn render_page<W: Write>(node: &Node<HTMLNode>, writer: &mut W) -> AstryxResult<()> {
    // can we avoid a clone here?
    Ok(match node.borrow().clone() {
        HTMLNode::Element(e) => {
            writer
                .write_str(&format!("{}", html_tag(&e.ident, &e.attributes)))
                .unwrap(); //todo: err
                
            for child in node.children() {
                render_page(&child, writer)?;
            }

            writer.write_str(&format!("</{}>", e.ident)).unwrap();
        }
        HTMLNode::Text(t) => {
            writer.write_str(&t).unwrap();
        }
    })
}

pub fn html_tag(ident: &str, attributes: &HashMap<String, String>) -> String {
    let attribs = if !attributes.is_empty() {
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
    };

    format!("<{}{}>", ident, attribs)
}

pub(crate) fn match_html_tag(
    ident: &str,
    locals: HashMap<String, String>,
) -> AstryxResult<HTMLNode> {
    match ident {
        "h1" | "h2" | "h3" | "abstract" | "hr" => Ok(HTMLNode::Element(HTMLElement {
            ident: ident.into(),
            attributes: HashMap::new(),
        })),
        "link" => {
            let mut attributes = HashMap::new();
            attributes.insert("href".into(), locals.get("path").expect("no path").clone());

            Ok(HTMLNode::Element(HTMLElement {
                ident: "a".into(),
                attributes,
            }))
        }
        "rows" | "row" => {
            let mut attributes = HashMap::new();
            attributes.insert("class".into(), ident.into());

            Ok(HTMLNode::Element(HTMLElement {
                ident: "div".into(),
                attributes,
            }))
        }
        _ => Err(AstryxError::new(&format!(
            "interpreter error: node not found: {}",
            ident
        ))),
    }
}
