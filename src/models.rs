

#[derive(Debug, Clone)]
pub enum Node {
    ForLoop(ForLoop),
    Element(Element),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub reference: String,
    pub iterable: Variable, // todo: Function<Variable>
}

#[derive(Debug, Clone)]
pub struct Element {
    pub ident: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Attribute {
    Symbol(String),
    Assignment {
        ident: String,
        variable: Variable,
    },
}

#[derive(Debug, Clone)]
pub enum Variable {
    RelativePath(String),
    QuotedString(String),
}
