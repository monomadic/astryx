/*
POSTPROCESSOR
- converts a graph of Nodes from a source tree into a set of rendered HTML pages
- resolves variables and scope
*/

use crate::{error::*, html::HTMLNode, imports::Imports};
use parser::{variable::Variable, Token, parser::Attribute};
use rctree::Node;
use std::collections::HashMap;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub(crate) fn run(tokens: &Vec<Token>) -> AstryxResult<HashMap<String, Node<HTMLNode>>> {
    let state = &mut State::new();

    for token in tokens {
        _run(token, state, &mut None)?;
    }

    Ok(state.pages.clone())
}

// TODO make private type
#[derive(Debug, Clone)]
pub struct State {
    local_variables: HashMap<String, Variable>,
    pages: HashMap<String, Node<HTMLNode>>,
    imports: Imports,
}

impl State {
    pub fn new() -> Self {
        State {
            local_variables: HashMap::new(),
            pages: HashMap::new(),
            imports: Imports::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    String(String),
    Document(Document),
    Array(Vec<Value>),
}

impl Value {
    fn from_variable(
        variable: &Variable,
        local_variables: &HashMap<String, Variable>,
    ) -> AstryxResult<Value> {
        Ok(match variable {
            Variable::QuotedString(s) => Value::String(s.clone()),
            Variable::RelativePath(_) => {
                // check file exists, load as document
                unimplemented!();
            }
            Variable::Reference(_) => {
                // resolve from local_variables
                unimplemented!();
            }
            Variable::TemplateFile(_) => {
                unimplemented!();
            } // delete this from original model
        })
    }

    fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            _ => panic!("oops"),
        }
    }
}

#[derive(Debug, Clone)]
struct Document {
    // created_at: Date
    pub body: String,
    // pub filename: String,
    // pub variables: HashMap<String, String>,
    pub metadata: Option<yaml_rust::Yaml>,
}

/// collect state and build a valid node tree
fn _run(token: &Token, state: &mut State, parent: &mut Option<Node<HTMLNode>>) -> AstryxResult<()> {
    match token {
        Token::Element(e) => {
            match e.ident.as_str() {
                // first check for system (static) functions
                "page" => {
                    let path: Variable = e.get_required_attribute("path")?;
                    let path: String =
                        Value::from_variable(&path, &state.local_variables)?.to_string();

                    // make a fresh node tree
                    let mut node = Node::new(HTMLNode::new_element("html"));
                    node.append(Node::new(HTMLNode::new_element("title")));

                    if let Some(stylesheet) = e.get_optional_attribute("stylesheet") {
                        let stylesheet: String =
                            Value::from_variable(&stylesheet, &state.local_variables)?.to_string();
                        node.append(Node::new(HTMLNode::new_stylesheet_element(stylesheet)));
                    }

                    let mut body = Some(Node::new(HTMLNode::new_element("body")));

                    for token in &e.children {
                        _run(token, state, &mut body)?;
                    }

                    node.append(body.unwrap()); // unwrap is ok cause I just made it Some... rethink this though

                    state.pages.insert(path, node.clone().root());
                }

                "embed" => {
                    let path: String = Value::from_variable(
                        &e.get_required_attribute("path")?,
                        &state.local_variables,
                    )?
                    .to_string();

                    let svgfile = crate::filesystem::read_file(std::path::PathBuf::from(path))?;
                    let node = Node::new(HTMLNode::Text(svgfile));

                    if let Some(parent) = parent {
                        parent.append(node);
                    } else {
                        return Err(AstryxError::new("tag found without page to assign to"));
                    }
                }

                _ => {
                    // must be a tag, lets try to resolve it

                    let mut el = state.imports.create_element(&e.ident)?;
                    // println!("GENERATED EL: {:?}", html_el);

                    // let mut el = crate::html::match_html_tag(&e.ident, locals)?;

                    for attr in &e.attributes.clone() {
                        // el.apply_attribute(&attr)?;
                        match attr {
                            Attribute::Class(class) => el.add_class(class),
                            Attribute::Symbol(modifier) => {
                                state.imports.modify_element(&modifier, None, &mut el)?;
                            }
                            Attribute::NamedAttribute { ident, variable } => {
                                match variable {
                                    Variable::QuotedString(s) => {
                                        state.imports.modify_element(&ident, Some(&s), &mut el)?;
                                    }
                                    _ => {
                                        println!("variable {:?}", variable);
                                        unimplemented!();
                                    },
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
                        return Err(AstryxError::new("tag found without page to assign to"));
                    }
                }
            }
        }
        Token::ForLoop(f) => {
            let files = crate::filesystem::read_content_metadata(&f.iterable)?;
            for file in files {
                // create a new local state to pass down the tree
                let mut new_state = state.clone();

                new_state
                    .local_variables
                    .insert(f.index.clone(), Variable::TemplateFile(file));

                for token in &f.children {
                    _run(token, &mut new_state, parent)?;
                }

                // state.page_buffers = new_state.page_buffers; // kind of a dirty hack
                state.pages = new_state.pages;
            }
        }
        Token::Text(t) => {
            if let Some(parent) = parent {
                let buffer = crate::interpolator::interpolate(t, &state.local_variables)?;
                parent.append(Node::new(HTMLNode::Text(buffer)));
            }
        }
        Token::CodeBlock(_) => {}
    }

    Ok(())
}
