// writes an xml graph to a html string

use crate::error::{AstryxError, AstryxResult};
use rctree::Node;
use std::{fmt::Write, collections::HashMap};

#[derive(Debug, Clone)]
pub(crate) struct HTMLNode {
    ident: String,
    attributes: HashMap<String, String>,
}

impl HTMLNode {
    pub(crate) fn new(ident: &str) -> Self {
        Self {
            ident: ident.into(),
            attributes: HashMap::new(),
        }
    }
}

pub(crate) fn render_page<W: Write>(node: &Node<HTMLNode>, writer: &mut W) -> AstryxResult<()> {
    writer.write_str(&format!("{}", html_tag(&node.borrow().ident, &node.borrow().attributes))).unwrap(); //todo: err
    for child in node.children() {
        render_page(&child, writer)?;
    }
    writer.write_str(&format!("</{}>", node.borrow().ident)).unwrap();
    Ok(())
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
    _locals: HashMap<String, String>, // TODO use locals
) -> AstryxResult<HTMLNode> {
    println!("checking {}", ident);
    match ident {
        "h1" => {
            Ok(HTMLNode {
                ident: ident.into(),
                attributes: HashMap::new(),
            })
        }
        "rows" | "row" => {
            let mut attributes = HashMap::new();
            attributes.insert("class".into(), ident.into());

            Ok(HTMLNode {
                ident: "div".into(),
                attributes,
            })
        },
        _ => Err(AstryxError::new(&format!(
            "interpreter error: node not found: {}",
            ident
        ))),
    }
}
