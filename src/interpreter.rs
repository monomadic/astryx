use crate::error::*;
use crate::{html::HTMLNode, models::*};
use rctree::Node;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct State {
    variables_in_scope: HashMap<String, Variable>,
    overlays: HashMap<String, TagOverlay>,
    decorators: HashMap<String, TagDecorator>,
    pages: HashMap<String, Node<HTMLNode>>,
}

impl State {
    pub fn new() -> Self {
        State {
            variables_in_scope: HashMap::new(),
            // page_buffers: HashMap::new(),
            overlays: TagOverlay::defaults(),
            decorators: TagDecorator::defaults(),
            pages: HashMap::new(),
        }
    }

    pub fn render_pages(&self) -> AstryxResult<HashMap<String, String>> {
        let mut pages = HashMap::new();
        for (route, node) in &self.pages {
            let buf = &mut String::new();
            crate::html::render_page(&node, buf)?;
            pages.insert(route.clone(), buf.clone());
        }
        Ok(pages)
    }

    pub fn get_required_variable(&self, i: &str) -> AstryxResult<&Variable> {
        self.variables_in_scope
            .get(i)
            .ok_or(AstryxError::new(&format!(
                "variable not found: {}\nvariables in scope: {:?}",
                i, self.variables_in_scope
            )))
    }
}

// TODO result should be meaningful, do not accept or leak state.
pub fn __run(tokens: &Vec<Token>, state: &mut State) -> AstryxResult<()> {
    for token in tokens {
        _run(token, state, &mut None)?;
    }

    Ok(())
}

// // Looks for references inside arguments and resolves them against local variables
// fn resolve_references(
//     arguments: &HashMap<String, Variable>,
//     locals: &HashMap<String, Variable>,
// ) -> AstryxResult<HashMap<String, Variable>> {
//     let mut resolved: HashMap<String, Variable> = HashMap::new();

//     for (ident, variable) in arguments {
//         resolved.insert(ident.clone(), resolve_reference(variable, locals)?);
//     }

//     Ok(resolved)
// }

// // if a variable has any references in it, attempt to resolve it.
// fn resolve_reference(variable: &Variable, locals: &HashMap<String, Variable>) -> AstryxResult<Variable> {
//     Ok(match variable {
//         Variable::Reference(r) => {
//             locals.get(r)?.clone()
//         }
//         _ => variable.clone()
//     })
// }

// Converts a series of variables to strings
fn stringify_variables(
    variables: &HashMap<String, Variable>,
    locals: &HashMap<String, Variable>,
) -> AstryxResult<HashMap<String, String>> {
    let mut stringified: HashMap<String, String> = HashMap::new();

    for (ident, variable) in variables {
        stringified.insert(ident.clone(), 
         crate::interpolator::stringify_variable(variable, locals)?);
    }

    Ok(stringified)
}

fn get_required(ident: &str, variables: &HashMap<String, String>) -> AstryxResult<String> {
    variables.get(ident)
        .map(|v|v.into())
        .ok_or(AstryxError::new("variable not found"))
}

// resolve_reference(variable: Variable, locals: &HashMap<String, Variable>)

pub(crate) fn _run(
    token: &Token,
    state: &mut State,
    parent: &mut Option<Node<HTMLNode>>,
) -> AstryxResult<()> {
    match token {
        Token::Element(e) => {
            let arguments = convert_attributes_into_locals(&e.attributes, &state.decorators)?;
            // let locals = resolve_references(&arguments, &state.variables_in_scope)?;
            let locals = stringify_variables(&arguments, &state.variables_in_scope)?;

            match e.ident.as_str() {
                "page" => {
                    let path = get_required("path", &locals)?;

                    // make a fresh node tree
                    let mut node = Node::new(HTMLNode::new_element("html"));
                    node.append(Node::new(HTMLNode::new_element("title")));
                    node.append(Node::new(HTMLNode::new_stylesheet_element("/style.css")));
                    let mut body = Some(Node::new(HTMLNode::new_element("body")));

                    for token in &e.children {
                        _run(token, state, &mut body)?;
                    }

                    node.append(body.unwrap()); // unwrap is ok cause I just made it Some... rethink this though

                    state.pages.insert(path.into(), node.clone().root());
                }
                // "element" => {
                //     let ident = crate::interpolator::stringify_variable(
                //         &get_required_argument("ident", &arguments)?,
                //         &HashMap::new(),
                //     )?;

                //     let mut node = Some(Node::new(HTMLNode::new(&ident)));

                //     for token in &e.children {
                //         _run(token, state, &mut node)?;
                //     }

                //     if let Some(parent) = parent {
                //         parent.append(Node::new(HTMLNode::new(&ident)));
                //     } else {
                //         return Err(AstryxError::new("tag found without page to assign to"));
                //     }
                // }
                "embed" => {
                    let path = get_required("path", &locals)?;
                    let svgfile = crate::filesystem::read_file(std::path::PathBuf::from(path))?;
                    let node = Node::new(HTMLNode::Text(svgfile));

                    if let Some(parent) = parent {
                        parent.append(node);
                    } else {
                        return Err(AstryxError::new("tag found without page to assign to"));
                    }
                }
                _ => {
                    let mut node = Some(Node::new(crate::html::match_html_tag(&e.ident, locals)?));

                    for token in &e.children {
                        _run(token, state, &mut node)?;
                    }

                    if let Some(parent) = parent {
                        parent.append(node.unwrap());
                    } else {
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
                    .variables_in_scope
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
                let buffer = crate::interpolator::interpolate(t, &state.variables_in_scope)?;
                parent.append(Node::new(HTMLNode::Text(buffer)));
            }
        }
        Token::CodeBlock(_) => {}
    }

    Ok(())
}

//                     _ => {
//                         // tag was not found, lets check if it exists as an overlay
//                         if let Some(overlay) = state.overlays.clone().get(&e.ident) {
//                             // it was an overlay, lets resolve it and reparse
//                             let current_el = Element {
//                                 ident: overlay.ident.clone(),
//                                 attributes: e.attributes.clone(),
//                                 children: e.children.clone(), // ouch, we should try to find a way around cloning here
//                             };
//                             run(&vec![Token::Element(current_el)], state)?;
//                         } else {
//                             // ok it's really not found, return an error.
//                             return Err(AstryxError::new(&format!(
//                                 "interpreter error: node not found: {}",
//                                 e.ident
//                             )));
//                         }
//                     }
//                 }
//             }
//             Token::Text(t) => {
//                 let buffer = crate::interpolator::interpolate(t, &state.variables_in_scope)?;
//                 state.write_to_current_buffer(&buffer)?;
//             }
//             Token::CodeBlock(cb) => {
//                 state
//                     .variables_in_scope
//                     .insert(cb.ident.clone(), Variable::QuotedString(cb.content.clone()));
//             }
//         }
//     }

/// Takes attributes from a node (which can be @decorators or named=arguments) and returns
/// a hashmap of local variables.
fn convert_attributes_into_locals(
    attributes: &Vec<Attribute>,
    decorators: &HashMap<String, TagDecorator>,
) -> AstryxResult<HashMap<String, Variable>> {
    let mut named_attributes: HashMap<String, Variable> = HashMap::new();

    for attribute in attributes {
        match attribute {
            Attribute::NamedAttribute { ident, variable } => {
                let _ = named_attributes.insert(ident.clone(), variable.clone());
            }
            Attribute::Decorator(d) => {
                if let Some(decorator) = decorators.get(&d.ident) {
                    // FIXME this is crap, needs a way better solution
                    named_attributes.insert(
                        "class".into(),
                        Variable::QuotedString(decorator.classes.join(" ")),
                    );
                // for class in decorator.classes {

                // }
                } else {
                    return Err(AstryxError::new("no such decorator".into()));
                }
            }
            Attribute::Symbol(_) => {}
        }
    }
    Ok(named_attributes)
}

pub fn get_optional_variable(i: &str, locals: &HashMap<String, Variable>) -> Option<Variable> {
    locals
        .get(&String::from(i.clone()))
        .map(|v| v.clone().clone())
}

pub fn get_required_argument(
    i: &str,
    arguments: &HashMap<String, Variable>,
) -> AstryxResult<Variable> {
    arguments
        .get(&i.to_string())
        .map(|v| v.clone().clone())
        .ok_or(AstryxError::new(&format!(
            "argument not found: {}. arguments: {:?}",
            i, arguments
        )))
}

#[derive(Debug, Clone)]
struct TagOverlay {
    ident: String,
    classes: Vec<String>,
    // attributes: HashMap<String, Attribute>,
}

impl TagOverlay {
    fn defaults() -> HashMap<String, TagOverlay> {
        let mut overlays = HashMap::new();

        overlays.insert(
            "image".into(),
            TagOverlay {
                ident: "img".into(),
                classes: vec![],
            },
        );

        overlays.insert(
            "h1".into(),
            TagOverlay {
                ident: "h1".into(),
                classes: vec![],
            },
        );

        overlays.insert(
            "columns".into(),
            TagOverlay {
                ident: "div".into(),
                classes: vec!["rows".into()],
            },
        );

        overlays.insert(
            "rows".into(),
            TagOverlay {
                ident: "div".into(),
                classes: vec!["rows".into()],
            },
        );

        overlays.insert(
            "row".into(),
            TagOverlay {
                ident: "div".into(),
                classes: vec!["row".into()],
            },
        );

        overlays
    }
}
#[derive(Debug, Clone)]
struct TagDecorator {
    classes: Vec<String>,
}

impl TagDecorator {
    fn defaults() -> HashMap<String, TagDecorator> {
        let mut decorators = HashMap::new();

        decorators.insert(
            "centered".into(),
            TagDecorator {
                classes: vec!["centered".into()],
            },
        );

        decorators.insert(
            "text-centered".into(),
            TagDecorator {
                classes: vec!["text-centered".into()],
            },
        );

        decorators.insert(
            "red".into(),
            TagDecorator {
                classes: vec!["red".into()],
            },
        );

        decorators
    }
}
