// eventually, introduce a split() function that walks the tree and pulls out pages
// so that all pages aren't rendered at once on the webserver frontend.

use crate::Object;
use error::AstryxResult;
use html::{render_document, HTMLNode};
use rctree::Node;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Site {
    /// html page trees
    pub pages: HashMap<String, Node<HTMLNode>>,

    /// static files
    pub files: HashMap<String, Vec<u8>>,
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
    match node.borrow().clone() {
        Object::None => {
            for child in node.children() {
                walk_nodes(child, cursor.clone(), path.clone(), site);
            }
        }
        Object::HTMLElement(el) => {
            let new_child = Node::new(HTMLNode::Element(el));

            cursor.append(new_child.clone());

            for child in node.children() {
                walk_nodes(child, new_child.clone(), path.clone(), site);
                // walk_nodes(child, cursor.make_deep_copy(), path.clone(), site);
            }
        }
        Object::String(s) => {
            let new_child = Node::new(HTMLNode::Text(s));

            cursor.append(new_child.clone());

            for child in node.children() {
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
        Object::File(path) => {
            // fixme: return AstryxError, don't panic
            let file =
                std::fs::read(path.clone()).expect(&format!("file to be readable: {}", path));

            site.files.insert(path.clone(), file);
        }
        _ => todo!("object not supported: {:?}", &node),
    }

    // println!("returning {:?}", render_document(&cursor.clone()));
    // println!("document {:?}", render_document(&cursor.root().clone()));
    cursor
}

impl Site {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
            files: HashMap::new(),
        }
    }

    pub fn render_pages(&self) -> HashMap<String, String> {
        self.pages
            .iter()
            .map(|(path, node)| {
                (
                    path.clone(),
                    write_html5_page_with_template(
                        "title",
                        &render_document(node),
                        r#"<link rel: "stylesheet", href: "style.css" />"#,
                        "description",
                        "author",
                    ),
                )
            })
            .collect()
    }

    pub fn render_as_bytes(&self) -> HashMap<String, Vec<u8>> {
        let mut blobs = HashMap::new();

        for (path, node) in &self.pages {
            blobs.insert(
                path.clone(),
                write_html5_page_with_template(
                    "title",
                    &render_document(node),
                    r#"<link rel: "stylesheet", href: "style.css" />"#,
                    "description",
                    "author",
                )
                .as_bytes()
                .to_vec(),
            );
        }

        for (path, file) in &self.files {
            blobs.insert(format!("/{}", path), file.clone());
        }

        blobs
    }

    /// write all files to disk
    pub fn write<P: AsRef<Path>>(&self, output_dir: P) -> AstryxResult<()> {
        for (route, document) in &self.pages {
            let output_dir = PathBuf::from("./")
                .join(output_dir.as_ref())
                .join(format!("./{}", route));
            let output_file_path = output_dir.join("index.html");

            println!("writing {}", output_file_path.to_str().unwrap());

            std::fs::create_dir_all(output_dir)?;
            std::fs::write(
                output_file_path,
                write_html5_page_with_template(
                    "title",
                    &render_document(document),
                    r#"<link rel: "stylesheet", href: "style.css" />"#,
                    "description",
                    "author",
                ),
            )?;
        }
        for (route, document) in &self.files {
            let file = PathBuf::from("./")
                .join(output_dir.as_ref())
                .join(format!("./{}", route));

            println!("writing {}", file.to_str().unwrap());

            std::fs::write(file, document)?;
        }
        Ok(())
    }
}

fn write_html5_page_with_template(
    title: &str,
    body: &str,
    header: &str,
    description: &str,
    author: &str,
) -> String {
    include_str!("../../../cli/templates/html5.html")
        .replace("$TITLE", title)
        .replace("$BODY", body)
        .replace("$HEADER", header)
        .replace("$DESCRIPTION", description)
        .replace("$AUTHOR", author)
}
