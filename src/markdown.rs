use crate::error::*;
use crate::highlighter::SyntaxHighlighter;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};

pub fn parse(i: &str) -> AstryxResult<String> {
    let mut h = SyntaxHighlighter::new();

    let parser = Parser::new_ext(i, Options::empty())
        .map(|event| match event {
            Event::Start(Tag::CodeBlock(ref kind)) => {
                match kind {
                    CodeBlockKind::Indented => (),
                    CodeBlockKind::Fenced(info) => {
                        h.set_syntax_by_file_extension(info);
                    }
                };

                let mut html = h.start_highlight();
                html.push_str("<code>");
                Event::Html(html.into())
            }
            Event::Text(text) => Event::Html(h.highlight_line(&text.to_owned()).into()),
            Event::End(Tag::CodeBlock(_)) => {
                Event::Html("</pre></code>".into())
            }

            _ => event,
        })
        .filter(|event| match event {
            Event::Start(Tag::Image(..)) | Event::End(Tag::Image(..)) => false,
            _ => true,
        });

    let mut buf: Vec<u8> = Vec::new();
    html::write_html(&mut buf, parser).unwrap();
    Ok(String::from_utf8(buf).unwrap())
}
