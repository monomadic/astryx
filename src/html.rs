// writes an xml graph to a html string

use std::collections::HashMap;
use rctree::Node;

#[derive(Debug, Clone)]
pub(crate) struct HTMLNode {
    ident: String,
    attributes: HashMap<String, String>
}

impl HTMLNode {
    pub(crate) fn new(ident: &str) -> Self {
        Self {
            ident: ident.into(),
            attributes: HashMap::new(),
        }
    }
}

pub(crate) fn render_page(node: &Node<HTMLNode>) {
    println!("<{}>", node.borrow().ident);
    for child in node.children() {
        render_page(&child)
    }
    println!("</{}>", node.borrow().ident);
}
