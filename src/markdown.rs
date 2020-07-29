use crate::error::*;
use crate::highlighter::SyntaxHighlighter;
use pulldown_cmark::{html, Event, Options, Parser, Tag, CodeBlockKind};

pub fn parse(i: &str) -> AstryxResult<String> {
    // TODO use a stricter lib that will throw errors, or
    // write one that returns a syntax tree of nodes
    // let parser = Parser::new_ext(i, Options::empty());
    // let mut syntax_mode = None;
    // let ps = SyntaxSet::load_defaults_newlines();
    // let ts = ThemeSet::load_defaults();
    // let syntax = ps.find_syntax_by_extension("rs").unwrap();
    // let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    // use syntect::easy::HighlightLines;
    // use syntect::parsing::SyntaxSet;
    // use syntect::highlighting::{ThemeSet, Style};
    // use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

    // Load these once at the start of your program
    // let ps = SyntaxSet::load_defaults_newlines();
    // let ts = ThemeSet::load_defaults();

    // let syntax = ps.find_syntax_by_extension("rs").unwrap();
    // let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    let mut h = SyntaxHighlighter::new();

    let parser = Parser::new_ext(i, Options::empty())
        .map(|event| match event {
            Event::Start(Tag::CodeBlock(ref kind)) => {
                // h.set_syntax_by_file_extension(kind.);
                // println!("H: {:?}", h);

                match kind {
                    CodeBlockKind::Indented => (),
                    CodeBlockKind::Fenced(info) => {
                        // highlighter = Some(get_highlighter(info, &context.config));
                        // println!("{:?}", info);
                        h.set_syntax_by_file_extension(info);
                    }
                };

                let mut html = h.start_highlight();
                html.push_str("<code>");
                // syntax_mode = Some(kind.to_owned());
                Event::Html(html.into())
                // Event::Html("<code>".into())
            }
            Event::Text(text) => Event::Html(h.highlight_line(&text.to_owned()).into()),
            Event::End(Tag::CodeBlock(_)) => {
                // syntax_mode = None;
                Event::Html("</pre></code>".into())
            }

            // Event::Start(Tag::CodeBlock(ref kind)) => {
            //     // let theme = &THEME_SET.themes[&context.config.highlight_theme];
            //     let ps = SyntaxSet::load_defaults_newlines();
            //     let ts = ThemeSet::load_defaults();
            //     let syntax = ps.find_syntax_by_extension("rs").unwrap();
            //     // let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
            //     let snippet = start_highlighted_html_snippet(&ts.themes["base16-ocean.dark"]);
            //     let mut html = snippet.0;

            //     println!("TEXT: {:?}", kind);

            //     html.push_str("<code>");
            //     Event::Html(html.into())

            //     // match kind {
            //     //     CodeBlockKind::Indented => (),
            //     //     CodeBlockKind::Fenced(info) => {
            //     //         highlighter = Some(get_highlighter(info, &context.config));
            //     //     }
            //     // };
            //     // // This selects the background color the same way that start_coloured_html_snippet does
            //     // let color = theme
            //     //     .settings
            //     //     .background
            //     //     .unwrap_or(::syntect::highlighting::Color::WHITE);
            //     // background = IncludeBackground::IfDifferent(color);
            //     // let snippet = start_highlighted_html_snippet(theme);
            //     // let mut html = snippet.0;
            //     // html.push_str("<code>");
            //     // Event::Html(html.into())
            // }
            _ => {
                event
            }
        })
        .filter(|event| match event {
            Event::Start(Tag::Image(..)) | Event::End(Tag::Image(..)) => false,
            _ => true,
        });

    let mut buf: Vec<u8> = Vec::new();
    html::write_html(&mut buf, parser).unwrap();
    Ok(String::from_utf8(buf).unwrap())
}
