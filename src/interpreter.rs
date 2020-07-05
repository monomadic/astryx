use std::collections::HashMap;

use crate::error::*;
use crate::models::*;

#[derive(Debug, Clone)]
pub struct Site {
    // pub styles: HashMap<String, Style>,
    pub pages: HashMap<String, String>,
}

// #[derive(Debug, Clone)]
// pub enum Style {
//     // todo: separate only into valid styles eg TextStyle
//     BackgroundColor(String), // todo: Color
//     Custom(String),          // custom css eg "border: 1px solid red" etc
// }

#[derive(Debug, Clone)]
pub struct State {
    pub page_buffers: HashMap<String, String>,
    variables_in_scope: HashMap<String, Variable>,
    current_page_buffer: Option<String>,
}

impl State {
    pub fn new() -> Self {
        State {
            variables_in_scope: HashMap::new(),
            page_buffers: HashMap::new(),
            current_page_buffer: None, // TODO should be current_page, it's not the buffer.
        }
    }

    pub fn get_required_variable(&self, i: &str) -> ParseResult<&Variable> {
        self.variables_in_scope
            .get(i)
            .ok_or(CassetteError::ParseError(
                format!(
                    "variable not found: {}\nvariables in scope: {:?}",
                    i, self.variables_in_scope
                ),
            ))
    }

    pub fn get_current_page_buffer(&mut self) -> ParseResult<&mut String> {
        if let Some(current_page) = self.current_page_buffer.clone() {
            if let Some(current_page_buffer) = self.page_buffers.get_mut(&current_page) {
                return Ok(current_page_buffer);
            }
        }
        // TODO return error
        panic!("oop");
    }

    // TODO extract this out into a multibuffer design pattern
    pub fn create_buffer(&mut self, key: String) -> ParseResult<()> {
        self.page_buffers.insert(key.clone(), String::new()); // FIXME check for collisions!
        self.current_page_buffer = Some(key);
        Ok(())
    }

    pub fn write_to_current_buffer(&mut self, string: &str) -> ParseResult<()> {
        Ok(self.get_current_page_buffer()?.push_str(string))
    }
}

pub fn html_tag(ident: &str, _attributes: Vec<(&str, String)>) -> String {
    format!("<{}>", ident)
}

/// run the interpreter over a series of nodes
pub fn run(nodes: &Vec<Node>, state: &mut State) -> ParseResult<()> {
    for node in nodes {
        match node {
            Node::Element(e) => {
                // println!("{}", e.ident);
                let arguments = collect_named_attributes(&e.attributes)?;

                match e.ident.as_str() {
                    "page" => {
                        // keep note of current page
                        let current_page = state.current_page_buffer.clone();
                        let path = stringify_variable(
                            &get_required_argument("path", &arguments)?,
                            state
                        )?;

                        state.create_buffer(path)?;
                        state.write_to_current_buffer("<html><head>")?;
                        if let Some(title) = get_optional_variable("title", &arguments) {
                            state.write_to_current_buffer(&format!("<title>{}</title>", title))?;
                        };
                        state.write_to_current_buffer("<body>")?;
                        run(&e.children, state)?;
                        state.write_to_current_buffer("</body></html>")?;

                        // surrender page buffer after use to previous page buffer
                        state.current_page_buffer = current_page;
                        println!("{:?}", state.page_buffers);
                    }
                    "row" | "column" => {
                        state.write_to_current_buffer(&format!("<div class=\"{}\">", e.ident))?;
                        run(&e.children, state)?;
                        state.write_to_current_buffer("</div>")?;
                    }
                    "image" | "img" | "i" => {
                        let path = stringify_variable(&get_required_argument("path", &arguments)?, state)?;

                        state.write_to_current_buffer(&html_tag("img", vec![("src", path)]))?;
                    }
                    _ => {
                        // panic!("");
                    }
                }
            }
            Node::Text(t) => {
                state.write_to_current_buffer(&t)?;
            }
            Node::ForLoop(f) => {
                // FIXME: throw errors in error conditions, don't just fall through
                // FIXME: give a variable which can be interpolated

                let files = crate::filesystem::read_content_metadata(&f.iterable)?;
                for file in files {
                    // state.variables_in_scope.get_mut(f.iterable) = file;
                    // make a copy of the state
                    // inject new copy of state with child variables
                    let mut new_state = state.clone();
                    new_state
                        .variables_in_scope
                        .insert("post.path".into(), Variable::QuotedString("hello".into()));
                    run(&f.children, &mut new_state)?;
                    // restore state copy
                }
            }
            Node::CodeBlock(_) => {}
        }
    }

    Ok(())
}

pub fn collect_named_attributes(
    attributes: &Vec<Attribute>,
) -> ParseResult<HashMap<&String, &Variable>> {
    let mut named_attributes: HashMap<&String, &Variable> = HashMap::new();

    for attribute in attributes {
        match attribute {
            Attribute::Assignment { ident, variable } => {
                let _ = named_attributes.insert(ident, variable);

                // .ok_or(CassetteError::ParseError(format!(
                //     "duplicate assignment: {}",
                //     ident
                // )))?;
            }
            _ => (),
        }
    }
    Ok(named_attributes)
}

pub fn get_optional_variable(
    i: &str,
    attributes: &HashMap<&String, &Variable>,
) -> Option<Variable> {
    attributes
        .get(&String::from(i.clone()))
        .map(|v| v.clone().clone())
}

pub fn get_required_variable(
    i: &str,
    attributes: &HashMap<&String, &Variable>,
) -> ParseResult<Variable> {
    attributes
        .get(&String::from(i.clone()))
        .map(|v| v.clone().clone())
        .ok_or(CassetteError::ParseError(format!(
            "could not find variable: {}",
            i
        )))
}

pub fn get_required_argument(
    i: &str,
    arguments: &HashMap<&String, &Variable>,
) -> ParseResult<Variable> {
    arguments
        .get(&i.to_string())
        .map(|v| v.clone().clone())
        .ok_or(
            CassetteError::ParseError(
                format!("argument not found: {}. arguments: {:?}", i, arguments))
        )

    // stringify_variable(&get_required_variable(i, arguments)?, state)
}

pub fn stringify_variable(variable: &Variable, state: &State) -> ParseResult<String> {
    match variable {
        Variable::RelativePath(p) => Ok(p.clone()),
        Variable::Reference(p) => {
            // resolve the reference
            state
                .variables_in_scope
                .get(p)
                .ok_or(
                    CassetteError::ParseError(format!("reference_not_found: {} {:?}", &p, &state.variables_in_scope)))
                .and_then(|v| stringify_variable(v, state))
        }
        Variable::QuotedString(p) => Ok(p.clone()),
    }
}

/// returns a specific string from an attributes array or throws an error.
pub fn get_required_string(
    i: &str,
    attributes: &HashMap<&String, &Variable>,
) -> ParseResult<String> {
    match get_required_variable(i, attributes)? {
        Variable::QuotedString(s) => {
            return Ok(s.clone());
        }
        _ => {
            // TODO return Err 'wrong type'.
            panic!(format!("wrong type: {:?}", i));
        }
    }
}

// fn write_page_buffer(page: String, state: &mut State, nodes: &Vec<Node>) -> ParseResult<()> {
//     state.create_buffer(page)?;
//     state.write_to_current_buffer("<html>")?;
//     run(nodes, state)?;
//     state.write_to_current_buffer("</html>")
// }

// fn write_html_tag(ident: String, state: &mut State, nodes: &Vec<Node>) -> ParseResult<()> {
//     Err(CassetteError::ParseError("hi".into()))
// }
