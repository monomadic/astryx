// takes lexical output from the parser and produces
// structured HTMLNode trees for each page

use crate::{
    error::*,
    html::HTMLNode,
    parser::{Attribute, Token},
    processors::Imports,
    variable::{stringify_variables, Variable},
};
use rctree::Node;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct State {
    local_variables: HashMap<String, Variable>,
    pages: HashMap<String, Node<HTMLNode>>,
    imports: Imports,

    // deprecated
    decorators: HashMap<String, TagDecorator>,
}

impl State {
    pub fn new() -> Self {
        State {
            local_variables: HashMap::new(),
            pages: HashMap::new(),
            imports: Imports::new(),

            // deprecated
            decorators: TagDecorator::defaults(),
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
        self.local_variables.get(i).ok_or(AstryxError::new(&format!(
            "variable not found: {}\nlocal_variables: {:?}",
            i, self.local_variables
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

fn get_required(ident: &str, variables: &HashMap<String, String>) -> AstryxResult<String> {
    variables
        .get(ident)
        .map(|v| v.into())
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
            let locals = stringify_variables(&arguments, &state.local_variables)?;
            // let classes = collect_classes(&e.attributes);

            match e.ident.as_str() {
                // first check for system (static) functions

                "page" => {
                    let path = get_required("path", &locals)?;

                    // make a fresh node tree
                    let mut node = Node::new(HTMLNode::new_element("html"));
                    node.append(Node::new(HTMLNode::new_element("title")));

                    if let Some(stylesheet) = locals.get("stylesheet") {
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
                    // must be a tag, lets try to resolve it

                    let mut el = state.imports.create_element(&e.ident)?;
                    // println!("GENERATED EL: {:?}", html_el);

                    // let mut el = crate::html::match_html_tag(&e.ident, locals)?;

                    for attr in &e.attributes.clone() {
                        // el.apply_attribute(&attr)?;
                        match attr {
                            Attribute::Class(class) => el.add_class(class),
                            Attribute::Symbol(modifier) => {
                                state.imports.modify_element(modifier, None, &mut el)?;
                            }
                            Attribute::NamedAttribute {ident, variable} => {
                                match variable {
                                    Variable::QuotedString(s) => {
                                        state.imports.modify_element(ident, Some(s), &mut el)?;
                                    },
                                    _ => panic!("case not covered"),
                                };
                                
                            },
                            Attribute::Decorator(_) => panic!("decorators deprecated")
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
            Attribute::Class(_) => {}
        }
    }
    Ok(named_attributes)
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
