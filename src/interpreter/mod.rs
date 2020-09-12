/// interpreter
/// - converts a graph of Nodes from a source tree into a set of rendered HTML pages
/// - resolves variables and scope
use crate::{
    error::*,
    html::{new_node_with_text, HTMLNode},
};
use arguments::{NamedArguments, TypeGetters};
use parser::{parser::Attribute, Token};
use rctree::Node;
use state::State;
use std::collections::HashMap;
use value::{Document, Value};

mod arguments;
mod state;
mod value;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub(crate) fn run(tokens: &Vec<Token>) -> AstryxResult<HashMap<String, Node<HTMLNode>>> {
    let state = &mut State::new();

    for token in tokens {
        _run(token, state, &mut None)?;
    }

    Ok(state.pages.clone())
}

/// recurse each token, resolve variables
fn _run(token: &Token, state: &mut State, parent: &mut Option<Node<HTMLNode>>) -> AstryxResult<()> {
    match token {
        Token::Comment(_) => {}
        Token::Element(e) => {
            let mut el = state.imports.create_element(&e.ident)?;

            for attr in &e.attributes.clone() {
                match attr {
                    // class attribute eg .blah
                    Attribute::Class(class) => el.add_class(class),
                    // symbol eg. centered align.center
                    Attribute::Symbol(_) => {
                        // state.imports.modify_element(&modifier, None, &mut el)?;
                        // ()
                        unimplemented!();
                    }
                    // named attribute eg. href="/index.html"
                    Attribute::NamedAttribute { ident, variable } => {
                        let value = state.resolve(variable)?;
                        state.imports.modify_element(
                            &ident,
                            Some(&String::from(value)),
                            &mut el,
                        )?;
                    }
                    // anonymous attribute eg disabled
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
                    "Anonymous tag: html tags must be part of a page or partial: {}",
                    e.ident
                )));
            }
        }
        Token::ForLoop(f) => {
            let path: Value = state.resolve(&f.iterable)?;

            if let Value::Path(path) = path {
                // this should eventually return an array type, not a vec<document>
                let documents = Document::read_from_glob(&path)?;

                if documents.len() == 0 {
                    return Err(AstryxError {
                        kind: AstryxErrorKind::FilesNotFound(path),
                        msg: format!("Could not find any files at {:?}", f.iterable),
                    });
                }

                // for loops should not assume documents in future...
                for document in documents {
                    // create a new local state to pass down the tree
                    let mut new_state = state.clone();

                    new_state.insert(&f.index, &Value::Document(document));

                    for token in &f.children {
                        _run(token, &mut new_state, parent)?;
                    }

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
            let arguments: NamedArguments = f
                .arguments
                .iter()
                .flat_map(|(ident, v)| state.resolve(v).map(|v| (ident.clone(), v.clone())))
                .collect();

            match f.ident.as_str() {
                "page" => {
                    // let el = functions::page(
                    //     arguments.get_required_string("route")?,
                    //     arguments.get_string("title")
                    // )?;

                    let route = arguments.get_required_string("route")?;
                    let stylesheet = arguments.get_string("stylesheet");

                    // make a fresh node tree
                    let mut node = Node::new(HTMLNode::new_element("html"));

                    // <title>
                    if let Some(title) = arguments.get_string("title") {
                        node.append(new_node_with_text("title", &title)?);
                    }

                    // <link rel="stylesheet">
                    if let Some(stylesheet) = stylesheet {
                        node.append(Node::new(HTMLNode::new_stylesheet_element(format!(
                            "/{}",
                            stylesheet
                        ))));
                    }

                    let mut body = Some(Node::new(HTMLNode::new_element("body")));

                    for token in &f.children {
                        _run(token, state, &mut body)?;
                    }

                    node.append(body.unwrap()); // unwrap is ok cause I just made it Some... rethink this though

                    state.pages.insert(route, node.clone().root());
                }
                "embed" => {
                    if let Some(parent) = parent {
                        let path: String = arguments.get_required_path("path")?;
                        println!("PATH: {:?}", path);
                        let svgfile: String = crate::filesystem::read_file(&path)?;
                        println!("NODE: {:?}", svgfile);
                        let node: Node<HTMLNode> = Node::new(HTMLNode::Text(svgfile));

                        parent.append(node);
                    } else {
                        return Err(AstryxError::new_from(AstryxErrorKind::UnexpectedFunction(
                            String::from("embed"),
                        )));
                    }
                }
                "exec" => unimplemented!(),
                _ => unimplemented!(),
            }
        }
    }

    Ok(())
}
