// eventually, introduce a split() function that walks the tree and pulls out pages
// so that all pages aren't rendered at once on the webserver frontend.

use crate::Object;
use html::{render_document, HTMLElement, HTMLNode};
use rctree::Node;
use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Site {
    // pub documents: HashMap<String, String>,
    pub pages: HashMap<String, Node<HTMLNode>>,
}

impl Into<Site> for Vec<Node<Object>> {
    fn into(self) -> Site {
        let mut site = Site::new();

        for node in self.clone() {
            walk_nodes(node, String::from("/"), &mut site);
        }

        for (path, page) in site.pages.iter() {
            println!("\n{}:\n{}", &path, render_document(page))
        }

        site
    }
}

/// Organise each node into pages, styles, and scripts.
fn walk_nodes(node: Node<Object>, mut path: String, site: &mut Site) {
    // entry
    match node.borrow().clone() {
        Object::None => {}
        Object::HTMLElement(el) => {
            if let Some(page_node) = site.pages.get_mut(&path) {
                // todo: wrap this in fn
                page_node.append(Node::new(HTMLNode::Element(el)));
            } else {
                site.pages
                    .insert(path.clone(), Node::new(HTMLNode::Element(el)));
            }
        }
        Object::HTMLPage(p) => path = p,
        Object::String(s) => {
            if let Some(page_node) = site.pages.get_mut(&path) {
                page_node.append(Node::new(HTMLNode::Text(s)));
            } else {
                site.pages
                    .insert(path.clone(), Node::new(HTMLNode::Text(s)));
            }
        }
        _ => println!("node found {:?}", &node),
    }

    // children
    for child in node.children() {
        walk_nodes(child, path.clone(), site);
    }

    // Node::new(HTMLElement::new("hi", Default::default()).unwrap())
}

impl Site {
    pub fn new() -> Self {
        Self {
            // documents: HashMap::new(),
            pages: HashMap::new(),
        }
    }

    pub fn render_pages(&self) -> HashMap<String, String> {
        self.pages
            .iter()
            .map(|(path, node)| (path.clone(), render_document(node)))
            .collect()
    }

    /// todo: supply output path
    pub fn write(&self) {
        for (hash, document) in &self.pages {
            let path = format!("./build{}/index.html", hash); // todo: don't do this.
            let path = std::path::Path::new(&path);
            let prefix = path.parent().unwrap();

            println!("writing {:?}", path);

            std::fs::create_dir_all(prefix).unwrap();
            std::fs::write(path, render_document(document)).unwrap();
        }
    }
}
//
// fn walk_nodes(node: Node<Object>, buffer: &mut HashMap<String, String>, mut path: String) {
//     // entry
//     match node.borrow().clone() {
//         Object::None => {}
//         Object::String(s) => write_to_buffer(buffer, &path, &s),
//         Object::Number(_) => unimplemented!(),
//         Object::HTMLPage(p) => path = p,
//         Object::HTMLElement(el) => write_to_buffer(buffer, &path, &el.open_tag()),
//         Object::BuiltinFunction(_) => unimplemented!(), // todo: why is this here?
//         Object::Array(arr) => {
//             for item in arr {
//                 walk_nodes(item, buffer, path.clone());
//             }
//         }
//         Object::Map(_) => unimplemented!(),
//         Object::Path(_) => unimplemented!(),
//         _ => unimplemented!(),
//     };
//
//     // children
//     for child in node.children() {
//         walk_nodes(child, buffer, path.clone());
//     }
//
//     // exit
//     match node.borrow().clone() {
//         Object::HTMLElement(el) => write_to_buffer(buffer, &path, &el.close_tag()),
//         _ => (),
//     }
// }
//
// fn write_to_buffer(buffer: &mut HashMap<String, String>, path: &str, content: &str) {
//     if let Some(page) = buffer.get_mut(path) {
//         *page = [page, content].concat();
//     } else {
//         buffer.insert(String::from(path), content.into());
//     }
// }
