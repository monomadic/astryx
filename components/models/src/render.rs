// eventually, introduce a split() function that walks the tree and pulls out pages
// so that all pages aren't rendered at once on the webserver frontend.

use crate::Object;
use rctree::Node;
use std::collections::HashMap;

pub struct Site {
    pub documents: HashMap<String, String>,
}

impl Site {
    pub fn render(nodes: Vec<Node<Object>>) -> Site {
        let mut documents = HashMap::new();

        for node in nodes {
            walk_nodes(node, &mut documents, String::from("/"));
        }

        Site { documents }
    }

    /// todo: supply output path
    pub fn write(&self) {
        for (hash, document) in &self.documents {
            let path = format!("./build{}/index.html", hash);
            let path = std::path::Path::new(&path);

            println!("writing {:?}", path);
            let prefix = path.parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();

            std::fs::write(path, document).unwrap();
        }
    }
}

fn walk_nodes(node: Node<Object>, buffer: &mut HashMap<String, String>, mut path: String) {
    // entry
    match node.borrow().clone() {
        Object::None => {}
        Object::String(s) => write_to_buffer(buffer, &path, &s),
        Object::Number(_) => unimplemented!(),
        Object::HTMLPage(p) => path = p,
        Object::HTMLElement(el) => write_to_buffer(buffer, &path, &el.open_tag()),
        Object::BuiltinFunction(_) => unimplemented!(),
        Object::Array(arr) => {
            for item in arr {
                walk_nodes(item, buffer, path.clone());
            }
        }
        Object::Map(_) => unimplemented!(),
    };

    // children
    for child in node.children() {
        walk_nodes(child, buffer, path.clone());
    }

    // exit
    match node.borrow().clone() {
        Object::HTMLElement(el) => write_to_buffer(buffer, &path, &el.close_tag()),
        _ => (),
    }
}

fn write_to_buffer(buffer: &mut HashMap<String, String>, path: &str, content: &str) {
    if let Some(page) = buffer.get_mut(path) {
        *page = [page, content].concat();
    } else {
        buffer.insert(String::from(path), content.into());
    }
}
