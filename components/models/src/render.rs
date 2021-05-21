// eventually, introduce a split() function that walks the tree and pulls out pages
// so that all pages aren't rendered at once on the webserver frontend.

use crate::Object;
use html::{render_document, HTMLNode};
use rctree::Node;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct Site {
    pub pages: HashMap<String, Node<HTMLNode>>,
}

impl Into<Site> for Vec<Node<Object>> {
    fn into(self) -> Site {
        let mut site = Site::new();

        // fixme: if this is an array, all trees will be rendered as /
        for node in self {
            let default_path = String::from("/");

            let root_node = walk_nodes(
                node,
                Node::new(HTMLNode::Root),
                default_path.clone(),
                &mut site,
            );

            site.pages.insert(default_path, root_node);
        }

        site
    }
}

fn walk_nodes(
    node: Node<Object>,
    mut cursor: Node<HTMLNode>,
    path: String,
    mut site: &mut Site,
) -> Node<HTMLNode> {
    // println!("site: {:?}", site.pages);
    // println!("node: {:?}", node);

    // if let Some(collision) = site.pages.get(&path.clone()) {
    //     // collision found
    // } else {
    // }

    // println!("walking {:?} ", render_document(&cursor));

    match node.borrow().clone() {
        Object::None => {
            for child in node.children() {
                // println!("child {:?}", child);
                walk_nodes(child, cursor.clone(), path.clone(), site);
            }
        }
        Object::HTMLElement(el) => {
            let new_child = Node::new(HTMLNode::Element(el));
            // println!("appending child: {:?}", render_document(&new_child));

            cursor.append(new_child.clone());
            // println!("appended: {:?}", render_document(&cursor));

            // if let Some(parent) = site
            //     .pages
            //     .insert(path.clone(), Node::new(HTMLNode::Element(el)))
            // {
            //     // parent.append();
            // }

            // println!("el {:?}", el);
            // println!("-site: {:?}", site.pages);

            for child in node.children() {
                // println!("child {:?}", child);
                walk_nodes(child, new_child.clone(), path.clone(), site);
                // walk_nodes(child, cursor.make_deep_copy(), path.clone(), site);
            }

            // append_child_node(site, path.clone(), HTMLNode::Element(el));
            // println!("HTMLElement: {:?}", el);
            // cursor.append(Node::new(HTMLNode::Element(el)));
            // println!("render_document: {:?}", render_document(&cursor.root()));
        }
        Object::String(s) => {
            let new_child = Node::new(HTMLNode::Text(s));
            // println!("appending text child: {:?}", render_document(&new_child));

            cursor.append(new_child.clone());
            // println!("appended text {:?}", render_document(&cursor));
            // println!(
            //     "appended text document {:?}",
            //     render_document(&cursor.root())
            // );

            // println!("text {:?} {:?}", cursor, new_child);

            for child in node.children() {
                // println!("child {:?}", child);
                walk_nodes(child, new_child.clone(), path.clone(), site);
            }
        }
        // Object::Path(s) => println!("path: {:?}", s),
        // Object::HTMLPage(s) => println!("change path: {:?}", s),
        // _ => println!("other {:?}", node),
        Object::HTMLPage(path) => {
            for page_node in node.children() {
                let root_node = walk_nodes(
                    page_node.clone(),
                    Node::new(HTMLNode::Root),
                    path.clone(),
                    &mut site,
                );
                // fixme: don't insert, collect the children together
                site.pages.insert(path.clone(), root_node);
            }
        }
        _ => todo!(),
    }

    // println!("returning {:?}", render_document(&cursor.clone()));
    // println!("document {:?}", render_document(&cursor.root().clone()));
    cursor
}
//
// /// Organise each node into pages, styles, and scripts.
// fn wwalk_nodes(
//     mut node: Node<Object>,
//     mut cursor: Node<HTMLNode>,
//     mut path: String,
//     site: &mut Site,
// ) {
//     println!("object: Object::{:?}", node);
//     println!("cursor: HTMLNode::{:?}", cursor);
//     println!("render_document {:?}", render_document(&cursor));
//
//     match node.borrow().clone() {
//         Object::None => {}
//         Object::HTMLElement(el) => {
//             let mut new_node = Node::new(HTMLNode::Element(el));
//             cursor.append(new_node.make_deep_copy());
//             // cursor = new_node.make_deep_copy();
//             site.pages.insert(path.clone(), new_node.make_deep_copy());
//             println!(" ** HTMLNode::{:?}", &cursor);
//
//             for child in node.children() {
//                 walk_nodes(child, new_node.make_deep_copy(), path.clone(), site);
//             }
//         }
//         Object::HTMLPage(p) => path = p,
//         Object::String(s) => {
//             let mut new_node = Node::new(HTMLNode::Text(s));
//             cursor.append(new_node.make_deep_copy());
//             // cursor = new_node.make_deep_copy();
//             site.pages.insert(path.clone(), new_node.make_deep_copy());
//             println!(" *3 HTMLNode::{:?}", &cursor);
//
//             for child in node.children() {
//                 walk_nodes(child, new_node.make_deep_copy(), path.clone(), site);
//             }
//         }
//         _ => println!("node found {:?}", &node),
//     }
//     println!(
//         "render_document post {} {:?}",
//         path,
//         render_document(&cursor)
//     );
// }

impl Site {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
        }
    }

    pub fn render_pages(&self) -> HashMap<String, String> {
        self.pages
            .iter()
            .map(|(path, node)| (path.clone(), render_document(node)))
            .collect()
    }

    /// todo: supply output directory
    pub fn write<P: AsRef<Path>>(&self, path: P) {
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
