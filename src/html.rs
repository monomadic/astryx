// writes an xml graph to a html string

use crate::error::{AstryxError, AstryxResult};
use rctree::Node;
use std::{collections::HashMap, fmt::Write};

#[derive(Debug, Clone)]
pub(crate) enum HTMLNode {
    Element(HTMLElement),
    Text(String),
}

#[derive(Debug, Clone)]
pub(crate) struct HTMLElement {
    ident: String,
    attributes: HashMap<String, String>,
}

impl HTMLNode {
    pub(crate) fn new_element(ident: &str) -> Self {
        HTMLNode::Element(HTMLElement {
            ident: ident.into(),
            attributes: HashMap::new(),
        })
    }

    pub(crate) fn new_element_with_attributes(ident: &str, attributes: HashMap<String, String>) -> Self {
        HTMLNode::Element(HTMLElement {
            ident: ident.into(),
            attributes: attributes,
        })
    }

    pub(crate) fn new_stylesheet_element<S: Into<String>>(path: S) -> Self {
        let mut attributes = HashMap::new();
        attributes.insert("rel".into(), "stylesheet".into());
        attributes.insert("href".into(), path.into());

        HTMLNode::Element(HTMLElement {
            ident: "link".into(),
            attributes: attributes,
        })
    }
}

pub(crate) fn render_page<W: Write>(node: &Node<HTMLNode>, writer: &mut W) -> AstryxResult<()> {
    // can we avoid a clone here?
    Ok(match node.borrow().clone() {
        HTMLNode::Element(e) => {
            writer
                .write_str(&format!("{}", html_tag(&e.ident, &e.attributes)))
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

pub fn html_tag(ident: &str, attributes: &HashMap<String, String>) -> String {
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

pub(crate) fn match_html_tag(
    ident: &str,
    locals: HashMap<String, String>,
) -> AstryxResult<HTMLNode> {

    if HTML_TAGS.contains(&ident) {
        return Ok(HTMLNode::Element(HTMLElement {
            ident: ident.into(),
            attributes: locals, // TODO filter this to type-check attributes
        }));
    }

    match ident {
        "link" | "a" => {
            let mut attributes = HashMap::new();
            attributes.insert("href".into(), locals.get("path").expect("no path").clone());

            Ok(HTMLNode::Element(HTMLElement {
                ident: "a".into(),
                attributes,
            }))
        }
        "rows" | "row" | "columns" | "column" => {
            let mut attributes = locals.clone();

            let class = attributes
                .get("class".into())
                .map(|c| format!(" {}", &c))
                .unwrap_or(String::new());

            let classes = &format!("{}{}", ident, class);

            attributes.insert("class".into(), classes.clone());

            Ok(HTMLNode::Element(HTMLElement {
                ident: "div".into(),
                attributes,
            }))
        }
        _ => Err(AstryxError::new(&format!(
            "interpreter error: node not found: {}",
            ident
        ))),
    }
}

const HTML_TAGS: &'static [&'static str] = &[
	"a",
	"abbr",
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
