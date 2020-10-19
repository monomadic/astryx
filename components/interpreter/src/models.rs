use html::HTMLElement;

#[derive(Debug)]
pub enum AstryxNode {
    HTMLElement(HTMLElement),
    Root,
}

#[derive(Debug)]
pub enum Value {
    String(String),
}
