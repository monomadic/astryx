/// interpreter
/// - converts a graph of Nodes from a source tree into a set of rendered HTML pages
/// - resolves variables and scope
use crate::{error::*, html::HTMLNode};
use parser::{parser::Attribute, variable::Variable, Token};
use rctree::Node;
use state::State;
use std::collections::HashMap;
use std::fmt;
use arguments::{TypeGetters, NamedArguments};

mod state;
mod arguments;
mod functions;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub(crate) fn run(tokens: &Vec<Token>) -> AstryxResult<HashMap<String, Node<HTMLNode>>> {
    let state = &mut State::new();

    for token in tokens {
        _run(token, state, &mut None)?;
    }

    Ok(state.pages.clone())
}

#[derive(Debug, Clone)]
pub(crate) enum Value {
    String(String),
    Document(Document),
    Documents(Vec<Document>),
    // Array(Vec<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Document(doc) => write!(f, "{}", doc.body),
            Value::String(s) => write!(f, "{}", s),
            Value::Documents(d) => write!(f, "{:?}", d),
            // Value::Array(a) => write!(f, "{:?}", a),
        }
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        match value {
            Value::String(s) => s,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Document {
    // created_at: Date
    pub body: String,
    pub metadata: Option<yaml_rust::Yaml>,
}

impl Document {
    pub(crate) fn get(&self, ident: &str) -> Option<String> {
        if ident == "body" {
            return Some(self.body.clone());
        }

        self.metadata
            .clone()
            .map(move |metadata| metadata[ident].clone())
            .and_then(|s| s.as_str().map(|s| s.to_string()))
            .map(|s| s.to_string())
    }
}

/// recurse each token, resolve variables
fn _run(token: &Token, state: &mut State, parent: &mut Option<Node<HTMLNode>>) -> AstryxResult<()> {
    match token {
        Token::Comment(_) => {}
        Token::Element(e) => {
            // create the local modifiers chain.
            // for attribute in &e.attributes {
            //     match attribute {
            //         Attribute::Class(s) =>
            //     }
            //     println!("a: {:?}", attribute);
            // }

            match e.ident.as_str() {
                // TODO page should just be another element type or function.
                "page" => {
                    let path: Value = state.resolve(&e.get_required_attribute("path")?)?;

                    // todo: fix this, create map of values
                    // let arguments: HashMap<String, Value> = e.attributes
                    //     .iter()
                    //     .flat_map(|a| {
                    //         Ok(("aaa", state.resolve(a)))
                    //     })
                    //     .collect();

                    // make a fresh node tree
                    let mut node = Node::new(HTMLNode::new_element("html"));
                    node.append(Node::new(HTMLNode::new_element("title")));

                    if let Some(stylesheet) = e.get_optional_attribute("stylesheet") {
                        let stylesheet = state.resolve(&stylesheet)?;

                        node.append(Node::new(HTMLNode::new_stylesheet_element(stylesheet)));
                    }

                    let mut body = Some(Node::new(HTMLNode::new_element("body")));

                    for token in &e.children {
                        _run(token, state, &mut body)?;
                    }

                    node.append(body.unwrap()); // unwrap is ok cause I just made it Some... rethink this though

                    state.pages.insert(path.into(), node.clone().root());
                }

                // "embed" => {
                //     let path: Value = state.resolve(&e.get_required_attribute("path")?)?;

                //     // let svgfile = crate::filesystem::read_file(std::path::PathBuf::from(path))?;
                //     // let node = Node::new(HTMLNode::Text(svgfile));

                //     // if let Some(parent) = parent {
                //     //     parent.append(node);
                //     // } else {
                //     //     return Err(AstryxError::new("tag found without page to assign to"));
                //     // }
                // }

                // "exec" => {}
                _ => {
                    // TODO this whole process should be
                    // - resolve references to values
                    // - pass along entire thing, get back html element blind,
                    // - I don't want to see the html element implementations
                    // - should know it is valid on return.

                    // when creating a htmlelement, should have knowledge of its own type and what it supports

                    // must be a tag, lets try to resolve it
                    let mut el = state.imports.create_element(&e.ident)?;

                    // let arguments: HashMap<String, Value> = e.attributes
                    //     .iter()
                    //     .flat_map(|a| {
                    //         Err(AstryxError::new(&format!("attempted to call modifier with {:?}", a)))
                    //     })
                    //     .collect();

                    for attr in &e.attributes.clone() {
                        // note this is temporary until we fix the parser with new syntax
                        match attr {
                            // class attribute eg .blah
                            Attribute::Class(class) => el.add_class(class),
                            // symbol eg. centered align.center
                            Attribute::Symbol(modifier) => {
                                state.imports.modify_element(&modifier, None, &mut el)?;
                            }
                            Attribute::NamedAttribute { ident, variable } => {
                                match variable {
                                    Variable::QuotedString(s) | Variable::RelativePath(s) => {
                                        state.imports.modify_element(
                                            &ident,
                                            Some(s.into()),
                                            &mut el,
                                        )?;
                                    }
                                    _ => {
                                        // FIXME: error
                                        // return Err(AstryxError::new(&format!(
                                        //     "attempted to call modifier with {:?}",
                                        //     attr
                                        // )));
                                    }
                                };
                            }
                            Attribute::Decorator(_) => panic!("decorators deprecated"),
                        }
                    }

                    let mut node = Some(Node::new(HTMLNode::Element(el)));

                    // interpret children
                    for token in &e.children {
                        _run(token, state, &mut node)?;
                    }

                    if let Some(parent) = parent {
                        parent.append(node.unwrap());
                    } else {
                        // tag was found that isn't actually in any structure
                        return Err(AstryxError::new(format!(
                            "tag found without page to assign to: {}",
                            e.ident
                        )));
                    }
                }
            }
        }
        Token::ForLoop(f) => {
            // if the forloop iterator is a series of valid documents,
            if let Value::Documents(documents) = state.resolve(&f.iterable)? {
                if documents.len() == 0 {
                    return Err(AstryxError {
                        kind: AstryxErrorKind::FilesNotFound(format!("{:?}", f.iterable)),
                        msg: format!("Could not find any files at {:?}", f.iterable),
                    });
                }

                for document in documents {
                    // create a new local state to pass down the tree
                    let mut new_state = state.clone();

                    new_state.insert(&f.index, &Value::Document(document));

                    println!("INDEX {:?}", f.index);

                    for token in &f.children {
                        _run(token, &mut new_state, parent)?;
                    }

                    // state.page_buffers = new_state.page_buffers; // kind of a dirty hack
                    state.pages = new_state.pages;
                }
            } else {
                return Err(AstryxError::new(format!(
                    "iterable was not a document array: {}",
                    f.iterable
                )));
            }
        }
        Token::Text(t) => {
            if let Some(parent) = parent {
                // let buffer = crate::interpolator::interpolate(t, &state.local_variables)?;
                parent.append(Node::new(HTMLNode::Text(state.interpolate_string(t)?)));
            }
        }
        Token::CodeBlock(_) => {}
        Token::FunctionCall(f) => {

            // resolve any variables in function arguments
            let arguments: NamedArguments = f.arguments
                .iter()
                .flat_map(|(ident, v)| {
                    state
                        .resolve(v)
                        .map(|v| (ident.clone(), v.clone()))
                })
                .collect();

            match f.ident.as_str() {
                "page" => functions::page(
                    arguments.get_required_string("route")?, 
                    arguments.get_string("title")),
                // _ => unimplemented!()
                _ => ()
            }

            // make a fresh node tree
            let mut node = Node::new(HTMLNode::new_element("html"));
            node.append(Node::new(HTMLNode::new_element("title")));

            // if let Some(stylesheet) = e.get_optional_attribute("stylesheet") {
            //     let stylesheet = state.resolve(&stylesheet)?;

            //     node.append(Node::new(HTMLNode::new_stylesheet_element(stylesheet)));
            // }

            let mut body = Some(Node::new(HTMLNode::new_element("body")));

            for token in &f.children {
                _run(token, state, &mut body)?;
            }

            node.append(body.unwrap()); // unwrap is ok cause I just made it Some... rethink this though

            state.pages.insert(arguments.get_required_string("route")?, node.clone().root());
        }
    }

    Ok(())
}
