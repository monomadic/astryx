use html::HTMLNode;
use rctree::Node;

pub enum ProgramNode {
    Root,
    SetPwd(String),        // change working directory
    SetOutput(Output),     // change output device
    HTMLElement(HTMLNode), // print a html element to the output device
    CSSRule,               // add a css style rule to the style block
                           // ImageInclude,
                           // JavaScript,
                           // Command, // arbitrary loaders and converters, image optimisers? shell command?
}

pub enum Output {
    StdOut,
    File(String),
}

// /// render a program to disk or buffer map
// pub fn render(root: Node<ProgramNode>, options: Option<RenderOptions>) -> ProgramResult<HashMap<String, String>> {}
// pub struct RenderOptions {
//     write_to_disk: bool,
// }

pub trait Inspect {
    fn inspect(&self) -> String;
}

impl Inspect for Node<ProgramNode> {
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

impl ProgramNode {
    fn inspect(&self) -> String {
        match self {
            ProgramNode::Root => format!("root"),
            ProgramNode::HTMLElement(e) => match e {
                HTMLNode::Element(el) => format!("el:{}", el.open_tag()),
                HTMLNode::Text(s) => format!("el:txt {}", s),
            },
            ProgramNode::CSSRule => format!("css"),
            ProgramNode::SetPwd(_) => format!("set_pwd"),
            ProgramNode::SetOutput(_) => format!("set_output"),
        }
    }
}