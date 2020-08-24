/*
POSTPROCESSOR
- converts a graph of Nodes from a source tree into a set of rendered HTML pages
- resolves variables and scope
*/

use crate::{error::*, html::HTMLNode};
use parser::{variable::Variable, Token};
use rctree::Node;
use state::State;
use std::collections::HashMap;

mod state;

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
    Array(Vec<Value>),
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        match value {
            Value::String(s) => s,
            _ => unimplemented!()
        }
    }
}

// impl Value {
//     fn from_variable(
//         variable: &Variable,
//         local_variables: &HashMap<String, Variable>,
//     ) -> AstryxResult<Value> {
//         Ok(match variable {
//             Variable::QuotedString(s) => Value::String(s.clone()),
//             Variable::RelativePath(_) => {
//                 // check file exists, load as document
//                 unimplemented!();
//             }
//             Variable::Reference(_) => {
//                 // resolve from local_variables
//                 unimplemented!();
//             }
//             Variable::TemplateFile(_) => {
//                 unimplemented!();
//             } // delete this from original model
//         })
//     }

//     fn to_string(&self) -> String {
//         match self {
//             Value::String(s) => s.clone(),
//             _ => panic!("oops"),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub(crate) struct Document {
    // created_at: Date
    pub body: String,
    pub metadata: Option<yaml_rust::Yaml>,
}

/// recurse each token, resolve variables
fn _run(token: &Token, state: &mut State, parent: &mut Option<Node<HTMLNode>>) -> AstryxResult<()> {
    match token {
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

                    let path: Value = state.resolve(
                        e.get_required_attribute("path")?
                    )?;

                    // let path: String =
                    //     Value::from_variable(&path, &state.local_variables)?.to_string();

                    // make a fresh node tree
                    let mut node = Node::new(HTMLNode::new_element("html"));
                    node.append(Node::new(HTMLNode::new_element("title")));

                    if let Some(stylesheet) = e.get_optional_attribute("stylesheet") {
                        // let stylesheet: String =
                        //     Value::from_variable(&stylesheet, &state.local_variables)?.to_string();
                        let stylesheet = state.resolve(
                            e.get_required_attribute("stylesheet")?
                        )?;

                        node.append(Node::new(HTMLNode::new_stylesheet_element(stylesheet)));
                    }

                    let mut body = Some(Node::new(HTMLNode::new_element("body")));

                    for token in &e.children {
                        _run(token, state, &mut body)?;
                    }

                    node.append(body.unwrap()); // unwrap is ok cause I just made it Some... rethink this though

                    state.pages.insert(path.into(), node.clone().root());
                }

                "embed" => {
                    let path: Value = state.resolve(
                        e.get_required_attribute("path")?
                    )?;

                    // let svgfile = crate::filesystem::read_file(std::path::PathBuf::from(path))?;
                    // let node = Node::new(HTMLNode::Text(svgfile));

                    // if let Some(parent) = parent {
                    //     parent.append(node);
                    // } else {
                    //     return Err(AstryxError::new("tag found without page to assign to"));
                    // }
                }

                _ => {
                    // must be a tag, lets try to resolve it

                    let mut el = state.imports.create_element(&e.ident)?;
                    // println!("GENERATED EL: {:?}", html_el);

                    // let mut el = crate::html::match_html_tag(&e.ident, locals)?;


                    // assemble modifiers from attributes


                    


                    // for attr in &e.attributes.clone() {
                    //     // el.apply_attribute(&attr)?;
                    //     match attr {
                    //         Attribute::Class(class) => el.add_class(class),
                    //         Attribute::Symbol(modifier) => {
                    //             state.imports.modify_element(&modifier, None, &mut el)?;
                    //         }
                    //         Attribute::NamedAttribute { ident, variable } => {
                    //             match variable {
                    //                 Variable::QuotedString(s) => {
                    //                     state.imports.modify_element(&ident, Some(&s), &mut el)?;
                    //                 }
                    //                 _ => {
                    //                     println!("variable {:?}", variable);
                    //                     unimplemented!();
                    //                 },
                    //             };
                    //         }
                    //         Attribute::Decorator(_) => panic!("decorators deprecated"),
                    //     }
                    // }

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
                // let buffer = crate::interpolator::interpolate(t, &state.local_variables)?;
                parent.append(Node::new(HTMLNode::Text(
                    format!("{:#?}", t)
                )));
            }
        }
        Token::CodeBlock(_) => {}
    }

    Ok(())
}
