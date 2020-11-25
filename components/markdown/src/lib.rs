mod highlighter;

use crate::highlighter::SyntaxHighlighter;
use error::LibError;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};

pub fn parse(i: &str) -> Result<String, LibError> {
    let mut h = SyntaxHighlighter::new();

    let tokens =
        Parser::new_ext(i, Options::empty()).map(|event| match event.clone() {
            Event::Start(Tag::CodeBlock(ref kind)) => match kind {
                CodeBlockKind::Fenced(info) => {
                    h.set_syntax_by_file_extension(info);
                    let html = h.start_highlight();
                    Event::Html(html.into())
                }
                _ => Event::Html("<pre>".into()),
            },
            Event::End(Tag::CodeBlock(_)) => {
                let html = h.stop_highlight();
                Event::Html(html.into())
            }
            Event::Text(text) => {
                if h.is_highlighting {
                    Event::Html(h.highlight_line(&text.to_owned()).into())
                } else {
                    event
                }
            }
            _ => event,
        });

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, tokens);

    Ok(html_output)
}
