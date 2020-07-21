use crate::error::*;
use crate::models::*;
use std::collections::HashMap;

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
    overlays: HashMap<String, TagOverlay>,
    decorators: HashMap<String, TagDecorator>,
}

impl State {
    pub fn new() -> Self {
        State {
            variables_in_scope: HashMap::new(),
            page_buffers: HashMap::new(),
            current_page_buffer: None, // TODO should be current_page, it's not the buffer.
            overlays: TagOverlay::defaults(),
            decorators: TagDecorator::defaults(),
        }
    }

    pub fn get_required_variable(&self, i: &str) -> AstryxResult<&Variable> {
        self.variables_in_scope
            .get(i)
            .ok_or(AstryxError::new(&format!(
                "variable not found: {}\nvariables in scope: {:?}",
                i, self.variables_in_scope
            )))
    }

    /// Retrieve the current page from the state buffer
    pub fn get_current_page_buffer(&mut self) -> AstryxResult<&mut String> {
        let state = self.clone();
        self.current_page_buffer
            .clone()
            .and_then(move |current_page_buffer| self.page_buffers.get_mut(&current_page_buffer))
            .ok_or(AstryxError {
                kind: AstryxErrorKind::Unknown,
                state: Some(state),
                msg: format!("page buffer request error."),
            })
    }

    // TODO extract this out into a multibuffer design pattern
    pub fn create_buffer(&mut self, key: String) -> AstryxResult<()> {
        self.page_buffers.insert(key.clone(), String::new()); // FIXME check for collisions!
        self.current_page_buffer = Some(key);
        Ok(())
    }

    pub fn write_to_current_buffer(&mut self, string: &str) -> AstryxResult<()> {
        self.get_current_page_buffer().and_then(|pb| {
            pb.push_str(string);
            Ok(())
        })
    }
}

pub fn html_tag(ident: &str, attributes: Vec<(String, String)>) -> String {
    let attribs = if !attributes.is_empty() {
        format!(
            " {}",
            attributes
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect::<Vec<String>>()
                .join(" ")
        )
    } else {
        String::new()
    };

    format!("<{}{}>", ident, attribs)
}

/// run the interpreter over a series of nodes
pub fn run(nodes: &Vec<Node>, state: &mut State) -> AstryxResult<()> {
    for node in nodes {
        match node {
            Node::Element(e) => {
                let arguments = collect_attributes(&e.attributes, &state.decorators)?;

                match e.ident.as_str() {
                    // TODO make elements scriptable / programmable
                    // suggestion: nodes can 'resolve' to other nodes, ending in tag
                    "page" => {
                        // keep note of current page
                        let current_page = state.current_page_buffer.clone();
                        let path = crate::interpolation::stringify_variable(
                            &get_required_argument("path", &arguments)?,
                            &state.variables_in_scope,
                        )?;

                        state.create_buffer(path)?;
                        state.write_to_current_buffer("<html><head>")?;

                        // <title> tag
                        if let Some(title) = get_optional_variable("title", &arguments) {
                            let title = crate::interpolation::stringify_variable(
                                &title,
                                &state.variables_in_scope,
                            )?;

                            state.write_to_current_buffer(&format!("<title>{}</title>", title))?;
                        };

                        if state.page_buffers.get("/style.css").is_some() {
                            state.write_to_current_buffer(
                                r#"<link rel="stylesheet" media="all" href="/style.css"/>"#,
                            )?;
                        }

                        // <style> in head tag
                        // if let Some(css) = state.variables_in_scope.get("css") {
                        //     let css = crate::interpolation::stringify_variable(
                        //         &css,
                        //         &state.variables_in_scope,
                        //     )?;

                        //     state.write_to_current_buffer(&format!("<style>{}</style>", css))?;
                        // };
                        state.write_to_current_buffer("<body>")?;
                        run(&e.children, state)?;
                        state.write_to_current_buffer("</body></html>")?;

                        // surrender page buffer after use to previous page buffer
                        state.current_page_buffer = current_page;
                    }
                    "css" => {
                        let path = get_required_argument("path", &arguments)?.to_string()?;

                        // let path = crate::interpolation::stringify_variable(
                        //     &get_required_argument("path", &arguments)?,
                        //     &state.variables_in_scope,
                        // )?;

                        let cssfile = crate::filesystem::read_file(std::path::PathBuf::from(path))?;

                        state.page_buffers.insert("/style.css".into(), cssfile);
                    }
                    "row" | "column" => {
                        state.write_to_current_buffer(&format!("<div class=\"{}\">", e.ident))?;
                        run(&e.children, state)?;
                        state.write_to_current_buffer("</div>")?;
                    }
                    "clamp" => {
                        // clamp(<min>, <actual>, <max>)
                        let max_width = crate::interpolation::stringify_variable(
                            &get_required_argument("max-width", &arguments)?,
                            &state.variables_in_scope,
                        )?;

                        state.write_to_current_buffer(&format!(
                            "<div style=\"width: clamp(10px, {}, 1000px)\">",
                            max_width
                        ))?;
                        run(&e.children, state)?;
                        state.write_to_current_buffer("</div>")?;
                    }
                    "image" | "img" | "i" => {
                        // let path =
                        //     stringify_variable(&get_required_argument("path", &arguments)?, state)?;

                        let path = crate::interpolation::stringify_variable(
                            &get_required_argument("path", &arguments)?,
                            &state.variables_in_scope,
                        )?;

                        state.write_to_current_buffer(&html_tag(
                            "img",
                            vec![("src".into(), path)],
                        ))?;
                    }
                    "h1" | "h2" | "h3" | "p" | "ul" | "li" | "ol" | "style" | "div" | "strong"
                    | "hr" | "abstract" => {
                        let attributes = collect_attributes(&e.attributes, &state.decorators)?;

                        state.write_to_current_buffer(&format!(
                            "<{}{}>",
                            &e.ident,
                            &attributes
                                .iter()
                                .map(|(ident, variable)| { format!(" {}=\"{}\"", ident, variable) })
                                .collect::<Vec<String>>()
                                .join("")
                        ))?;
                        run(&e.children, state)?;
                        state.write_to_current_buffer(&format!("</{}>", e.ident))?;
                    }
                    "link" | "a" => {
                        let path = crate::interpolation::stringify_variable(
                            &get_required_argument("path", &arguments)?,
                            &state.variables_in_scope,
                        )?;

                        state.write_to_current_buffer(&format!("<a href=\"{}\">", path))?;
                        run(&e.children, state)?;
                        state.write_to_current_buffer("</a>")?;
                    }
                    "embed" => {
                        let path = crate::interpolation::stringify_variable(
                            &get_required_argument("path", &arguments)?,
                            &state.variables_in_scope,
                        )?;

                        let svgfile = crate::filesystem::read_file(std::path::PathBuf::from(path))?;

                        state.write_to_current_buffer(&svgfile)?;
                    }
                    "tag" => {
                        let attributes = collect_attributes(&e.attributes, &state.decorators)?;

                        state.write_to_current_buffer(&format!(
                            "<{}{}>",
                            &e.ident,
                            &attributes
                                .iter()
                                .map(|(ident, variable)| { format!(" {}=\"{}\"", ident, variable) })
                                .collect::<Vec<String>>()
                                .join("")
                        ))?;
                        run(&e.children, state)?;
                        state.write_to_current_buffer(&format!("</{}>", e.ident))?;
                    }
                    _ => {
                        // tag was not found, lets check if it exists as an overlay
                        if let Some(overlay) = state.overlays.clone().get(&e.ident) {
                            // it was an overlay, lets resolve it and reparse
                            let current_el = Element {
                                ident: overlay.ident.clone(),
                                attributes: e.attributes.clone(),
                                children: e.children.clone(), // ouch, we should try to find a way around cloning here
                            };
                            run(&vec![Node::Element(current_el)], state)?;
                        } else {
                            // ok it's really not found, return an error.
                            return Err(AstryxError::new(&format!(
                                "interpreter error: node not found: {}",
                                e.ident
                            )));
                        }
                    }
                }
            }
            Node::Text(t) => {
                let buffer = crate::interpolation::interpolate(t, &state.variables_in_scope)?;
                state.write_to_current_buffer(&buffer)?;
            }
            Node::ForLoop(f) => {
                // FIXME: throw errors in error conditions, don't just fall through
                // FIXME: give a variable which can be interpolated

                let files = crate::filesystem::read_content_metadata(&f.iterable)?;
                for file in files {
                    // create a new local state to pass down the tree
                    let mut new_state = state.clone();

                    new_state
                        .variables_in_scope
                        .insert(f.index.clone(), Variable::TemplateFile(file));

                    run(&f.children, &mut new_state)?;
                    state.page_buffers = new_state.page_buffers; // kind of a dirty hack
                }
            }
            Node::CodeBlock(cb) => {
                state
                    .variables_in_scope
                    .insert(cb.ident.clone(), Variable::QuotedString(cb.content.clone()));
            }
        }
    }

    Ok(())
}

fn collect_attributes(
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
            "rows".into(),
            TagOverlay {
                ident: "div".into(),
                classes: vec!["rows".into()],
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

        decorators
    }
}
