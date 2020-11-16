use html::HTMLNode;
use rctree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
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

impl ProgramNode {
    pub fn render_start(&self) -> String {
        match self {
            ProgramNode::Root => format!(""),
            ProgramNode::SetPwd(_) => format!(""),
            ProgramNode::SetOutput(_) => format!(""),
            ProgramNode::HTMLElement(e) => match e {
                HTMLNode::Element(e) => e.open_tag(),
                HTMLNode::Text(t) => t.clone(),
            },
            ProgramNode::CSSRule => format!(""),
        }
    }

    pub fn render_end(&self) -> String {
        match self {
            ProgramNode::HTMLElement(e) => match e {
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

// pub trait Render {
//     fn render_pages(&self) -> HashMap<String, String>;
//     // fn render_css(&self) -> String;
//     // fn render_js(&self) -> String;
//     // fn get_file_blobs(&self) -> HashMap<String, Vec<u8>>;
// }

// fn render_pages(nodes: Node<ProgramNode>) -> HashMap<String, String> {
//     let node = nodes.borrow();
//     let mut pages = HashMap::new();

//     pages.insert(
//         String::from("/"),
//         format!(
//             "{}{}{}",
//             node.render_start(),
//             nodes
//                 .children()
//                 .map(render_pages)
//                 .map(|(url, page)| page)
//                 .collect::<Vec<String>>()
//                 .join(""),
//             node.render_end()
//         ),
//     );

//     pages
// }

fn render_pages(
    node: Node<ProgramNode>,
    pwd: String,
    output: String,
    files: &mut HashMap<String, String>,
) {
    let n = node.borrow().clone();
    match n {
        ProgramNode::Root => println!("root"),
        ProgramNode::SetPwd(pwd) => render_pages(node, pwd.into(), output),
        ProgramNode::SetOutput(_) => unimplemented!(),
        ProgramNode::HTMLElement(_) => unimplemented!(),
        ProgramNode::CSSRule => unimplemented!(),
    }
}

pub fn create_filemap(node: Node<ProgramNode>) -> HashMap<String, String> {
    render_pages(node, String::from("/"), String::from("stdout"))
}
