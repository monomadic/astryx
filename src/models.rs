

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Element {
    pub ident: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub ident: String,
    // pub value: Option<VariableType>,
}
