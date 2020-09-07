/// interpreter
/// - converts a graph of Nodes from a source tree into a set of rendered HTML pages
/// - resolves variables and scope
use crate::{error::*, html::HTMLNode};
use parser::{parser::Attribute, variable::Variable, Token};
use rctree::Node;
use state::State;
use std::collections::HashMap;
use arguments::{TypeGetters, NamedArguments};
use value::Value;

mod state;
mod arguments;
mod functions;
mod value;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub(crate) fn run(tokens: &Vec<Token>) -> AstryxResult<HashMap<String, Node<HTMLNode>>> {
    let state = &mut State::new();

    for token in tokens {
        _run(token, state, &mut None)?;
    }

    Ok(state.pages.clone())
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
            match e.ident.as_str() {
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
                                let value = state.resolve(variable)?;
                                state.imports.modify_element(
                                    &ident,
                                    Some(&String::from(value)),
                                    &mut el,
                                )?;
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
                "page" => {
                    // let el = functions::page(
                    //     arguments.get_required_string("route")?, 
                    //     arguments.get_string("title")
                    // )?;

                    let route = arguments.get_required_string("route")?;
                    let title = arguments.get_string("title");
                    let stylesheet = arguments.get_string("stylesheet");

                    // make a fresh node tree
                    let mut node = Node::new(HTMLNode::new_element("html"));
                    node.append(Node::new(HTMLNode::new_element("title")));

                    if let Some(stylesheet) = stylesheet {
                        node.append(Node::new(HTMLNode::new_stylesheet_element(stylesheet)));
                    }

                    let mut body = Some(Node::new(HTMLNode::new_element("body")));

                    for token in &f.children {
                        _run(token, state, &mut body)?;
                    }

                    node.append(body.unwrap()); // unwrap is ok cause I just made it Some... rethink this though

                    state.pages.insert(route, node.clone().root());
                }
                "embed" => {
                    let path = arguments.get_required_path("path")?;

                    let svgfile = crate::filesystem::read_file(path)?;
                    let node = Node::new(HTMLNode::Text(svgfile));

                    if let Some(parent) = parent {
                        parent.append(node);
                    } else {
                        return Err(AstryxError::new("tag found without page to assign to"));
                    }
                }
                "exec" => unimplemented!(),
                _ => unimplemented!()
            }


        }
    }

    Ok(())
}
