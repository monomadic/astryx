// writes an xml graph to a html string

use crate::{error::{AstryxError, AstryxResult}};
use rctree::Node;
use std::{collections::HashMap, fmt::Write};

#[derive(Debug, Clone)]
pub(crate) enum HTMLNode {
    Element(HTMLElement),
    Text(String),
}

#[derive(Debug, Clone)]
pub(crate) struct HTMLElement {
    pub ident: String,
    pub(crate) attributes: HashMap<String, String>,
	pub classes: Vec<String>,
	pub styles: Vec<String>, // should be type safe
}

impl HTMLElement {

	pub(crate) fn new_from_html_tag<S:ToString>(ident: S) -> AstryxResult<HTMLElement> {
		if HTML_TAGS.contains(&&(*ident.to_string())) {
			Ok(HTMLElement {
				ident: ident.to_string(),
				attributes: HashMap::new(),
				classes: Vec::new(),
				styles: Vec::new(),
			})
		} else {
			Err(AstryxError::new(&format!(
				"no such tag or overlay: {}",
				ident.to_string()
			)))
		}
	}

	// pub(crate) fn new_with_class<S:Into<String>>(ident: S, class: S) -> HTMLElement {
	// 	HTMLElement {
	// 		ident: ident.into(),
	// 		attributes: HashMap::new(),
	// 		classes: vec![class.into()],
	// 		styles: Vec::new(),
	// 	}
	// }

	pub(crate) fn add_class<S:ToString>(&mut self, class: S) {
		self.classes.push(class.to_string())
	}

	pub(crate) fn add_style<S:ToString>(&mut self, class: S) {
		self.styles.push(class.to_string())
	}
}

impl HTMLNode {
    pub(crate) fn new_element(ident: &str) -> Self {
        HTMLNode::Element(HTMLElement {
            ident: ident.into(),
            attributes: HashMap::new(),
			classes: Vec::new(),
			styles: Vec::new(),
        })
    }

    pub(crate) fn new_stylesheet_element<S: Into<String>>(path: S) -> Self {
        let mut attributes = HashMap::new();
        attributes.insert("rel".into(), "stylesheet".into());
        attributes.insert("href".into(), path.into());

        HTMLNode::Element(HTMLElement {
            ident: "link".into(),
            attributes: attributes,
			classes: Vec::new(),
			styles: Vec::new(),
        })
    }
}

pub(crate) fn render_as_string(nodes: &HashMap<String, Node<HTMLNode>>) -> AstryxResult<HashMap<String, String>> {
	let mut pages: HashMap<String, String> = HashMap::new();

	for (route, node) in nodes {
		let buf = &mut String::new();
		crate::html::render_page(&node, buf)?;
		pages.insert(route.into(), buf.to_string());
	}
	Ok(pages)
}

pub(crate) fn render_page<W: Write>(node: &Node<HTMLNode>, writer: &mut W) -> AstryxResult<()> {
    // can we avoid a clone here?
    Ok(match node.borrow().clone() {
        HTMLNode::Element(e) => {
            let mut attributes = e.attributes.clone();
            attributes.insert("class".into(), e.classes.join(" "));
            writer
                .write_str(&format!("{}", html_tag(&e)))
                .unwrap(); //todo: err
                
            for child in node.children() {
                render_page(&child, writer)?;
            }

            writer.write_str(&format!("</{}>", e.ident)).unwrap();
        }
        HTMLNode::Text(t) => {
            writer.write_str(&t).unwrap();
        }
    })
}

/// render HTMLElement as string
fn html_tag(el: &HTMLElement) -> String {
	let mut el = el.clone();
	
    if !el.classes.is_empty() {
        el.attributes.insert("class".into(), el.classes.join(" "));
	}
	
	if !el.styles.is_empty() {
        el.attributes.insert("style".into(), el.styles.join(" "));
    }

	// format attributes
    let attribs = if !el.attributes.is_empty() {
        format!(
            " {}",
            el.attributes
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect::<Vec<String>>()
                .join(" "))
    } else {
        String::new()
    };

    format!("<{}{}>", el.ident, attribs)
}

const HTML_TAGS: &'static [&'static str] = &[
	"a",
    "abbr",
    "abstract",
	"address",
	"area",
	"article",
	"aside",
	"audio",
	"b",
	"base",
	"bdi",
	"bdo",
	"blockquote",
	"body",
	"br",
	"button",
	"canvas",
	"caption",
	"cite",
	"code",
	"col",
	"colgroup",
	"data",
	"datalist",
	"dd",
	"del",
	"details",
	"dfn",
	"dialog",
	"div",
	"dl",
	"dt",
	"em",
	"embed",
	"fieldset",
	"figcaption",
	"figure",
	"footer",
	"form",
	"h1",
	"h2",
	"h3",
	"h4",
	"h5",
	"h6",
	"head",
	"header",
	"hgroup",
	"hr",
	"html",
	"i",
	"iframe",
	"img",
	"input",
	"ins",
	"kbd",
	"label",
	"legend",
	"li",
	"link",
	"main",
	"map",
	"mark",
	"math",
	"menu",
	"menuitem",
	"meta",
	"meter",
	"nav",
	"noscript",
	"object",
	"ol",
	"optgroup",
	"option",
	"output",
	"p",
	"param",
	"picture",
	"pre",
	"progress",
	"q",
	"rb",
	"rp",
	"rt",
	"rtc",
	"ruby",
	"s",
	"samp",
	"script",
	"section",
	"select",
	"slot",
	"small",
	"source",
	"span",
	"strong",
	"style",
	"sub",
	"summary",
	"sup",
	"svg",
	"table",
	"tbody",
	"td",
	"template",
	"textarea",
	"tfoot",
	"th",
	"thead",
	"time",
	"title",
	"tr",
	"track",
	"u",
	"ul",
	"var",
	"video",
	"wbr"
];
