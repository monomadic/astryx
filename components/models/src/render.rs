// eventually, introduce a split() function that walks the tree and pulls out pages
// so that all pages aren't rendered at once on the webserver frontend.

use crate::Object;
use rctree::Node;
use std::collections::HashMap;

impl Object {
    pub fn render(nodes: Vec<Node<Object>>) -> HashMap<String, Vec<u8>> {
        let mut buffer = HashMap::new();

        for node in nodes {
            walk_nodes(node, &mut buffer, String::from("/"));
        }

        buffer
    }
}

fn walk_nodes(node: Node<Object>, buffer: &mut HashMap<String, Vec<u8>>, mut path: String) {
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

fn write_to_buffer(buffer: &mut HashMap<String, Vec<u8>>, path: &str, content: &str) {
    if let Some(page) = buffer.get_mut(path) {
        *page = [page, content.as_bytes()].concat();
    } else {
        buffer.insert(String::from(path), content.into());
    }
}
