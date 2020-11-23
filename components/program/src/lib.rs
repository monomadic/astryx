use html::HTMLNode;
use rctree::Node;

mod project;
pub use project::Project;

#[derive(Clone, Debug)]
pub enum ProgramInstruction {
    Root,
    SetPwd(String),        // change working directory
    SetPath(Output),       // change output path
    HTMLElement(HTMLNode), // print a html element to the output device
    Text(String),          // print text to current output
    CSSRule,               // add a css style rule to the style block
                           // ImageInclude,
                           // JavaScript,
                           // Command, // arbitrary loaders and converters, image optimisers? shell command?
}

#[derive(Clone, Debug)]
pub enum Output {
    StdOut,
    File(String),
}

// /// render a program to disk or buffer map
// pub fn render(root: Node<ProgramNode>, options: Option<RenderOptions>) -> ProgramResult<HashMap<String, String>> {}
// pub struct RenderOptions {
//     write_to_disk: bool,
// }

impl ProgramInstruction {
    pub fn render_start(&self) -> String {
        match self {
            ProgramInstruction::Root => format!(""),
            ProgramInstruction::SetPwd(_) => format!(""),
            ProgramInstruction::SetPath(_) => format!(""),
            ProgramInstruction::HTMLElement(e) => match e {
                HTMLNode::Element(e) => e.open_tag(),
                HTMLNode::Text(t) => t.clone(),
            },
            ProgramInstruction::CSSRule => format!(""),
            ProgramInstruction::Text(_) => format!(""),
        }
    }

    pub fn render_end(&self) -> String {
        match self {
            ProgramInstruction::HTMLElement(e) => match e {
                HTMLNode::Element(e) => e.close_tag(),
                _ => String::new(),
            },
            _ => String::new(),
        }
    }
}

pub trait Inspect {
    fn inspect(&self) -> String;
}

impl Inspect for Node<ProgramInstruction> {
    fn inspect(&self) -> String {
        format!(
            "({}{})",
            self.borrow().inspect(),
            self.children()
                .map(|c| format!(" {}", c.inspect()))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl ProgramInstruction {
    pub fn inspect(&self) -> String {
        match self {
            ProgramInstruction::Root => format!("root"),
            ProgramInstruction::HTMLElement(e) => match e {
                HTMLNode::Element(el) => format!("el:{}", el.open_tag()),
                HTMLNode::Text(s) => format!("el:txt {}", s),
            },
            ProgramInstruction::CSSRule => format!("css"),
            ProgramInstruction::SetPwd(_) => format!("set_pwd"),
            ProgramInstruction::SetPath(_) => format!("set_output"),
            ProgramInstruction::Text(_) => format!("Text"),
        }
    }
}

fn render_pages(
    nodes: &mut Vec<ProgramInstruction>,
    pwd: String,
    path: String,
    project: &mut Project,
) {
    if let Some(node) = nodes.pop() {
        match node {
            ProgramInstruction::Root => unimplemented!(),
            ProgramInstruction::SetPwd(pwd) => render_pages(nodes, pwd.into(), path, project),
            ProgramInstruction::SetPath(_) => unimplemented!(),
            ProgramInstruction::HTMLElement(_) => unimplemented!(),
            ProgramInstruction::CSSRule => unimplemented!(),
            ProgramInstruction::Text(text) => {
                project.write_page(&path, &text);
                render_pages(nodes, pwd, path, project)
            }
        };
    }

    // render children
    // for child in node.children() {}
}

pub fn render_project(mut program: Vec<ProgramInstruction>) -> Project {
    println!("program: {:?}", program);

    let mut project = Project::new();

    program.reverse();

    render_pages(
        &mut program,
        String::from("."),
        String::from("/"),
        &mut project,
    );

    project
}
