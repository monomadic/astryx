use std::io::Write;

use crate::HTMLElement;
use rctree::Node;

#[derive(Debug, Clone)]
pub enum Program {
    HTMLElement(HTMLElement),
    Text(String),
    // CopyFile(),
    // SetWriter(),
}

pub fn eval<W: Write>(node: Node<Program>, writer: Option<W>) {
    println!("writing!");
}
