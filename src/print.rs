use crate::models::*;

pub fn print_nodes(nodes: Vec<Node>, indent: usize) {
    // format!("\n{: >i$}.{} {:?}", "", k, v, i=indent+1)
    // print!("{: >i$}", "", i=indent);
    // print!("{: >i$}", "", i=indent);
    // print!("{: >i$}", "", i=indent);
    // print!("{: >i$}", "", i=indent);


    for node in nodes {
        // println!("printing with indent: {}", indent);
        match node {
            Node::Text(t) => println!("| {}", t),
            Node::Element(e) => print_element(e, indent),
        }
    }
}

pub fn print_indent(indent: usize) {
    let iter = std::iter::repeat(indent).take(indent);
    for i in iter {
        print!("{: >i$}", "", i=indent);
        print!("{: >i$}", "", i=indent);
        print!("{: >i$}", "", i=indent);
        print!("{: >i$}", "", i=indent);
    }
}

pub fn print_element(e: Element, indent: usize) {
    print_indent(indent);
    print!("{}", e.ident);
    for attribute in e.attributes {
        print_attribute(attribute);
    }
    print!("\n");
    print_nodes(e.children, indent+1);
}

pub fn print_attribute(a: Attribute) {
    match a {
        Attribute::Symbol(s) => print!(" {}", s),
        Attribute::Assignment {ident, variable} =>
            print!(" {}={}", ident, variable_to_string(variable)),
    }
}

pub fn variable_to_string(v: Variable) -> String {
    match v {
        Variable::QuotedString(s) => format!("\"{}\"", s),
        Variable::RelativePath(s) => format!("./{}", s),
    }
}