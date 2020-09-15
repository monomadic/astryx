// syntax highlighting for source text

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{
    start_highlighted_html_snippet, styled_line_to_highlighted_html, IncludeBackground,
};
use syntect::parsing::SyntaxSet;

pub struct SyntaxHighlighter {
    syntax: String, // todo: enum
    syntaxes: SyntaxSet,
    themes: ThemeSet,
    pub is_highlighting: bool,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        SyntaxHighlighter {
            themes: ThemeSet::load_defaults(),
            syntaxes: SyntaxSet::load_defaults_newlines(),
            syntax: String::from("rs"),
            is_highlighting: false,
        }
    }

    pub fn set_syntax_by_file_extension(&mut self, ext: &str) {
        self.syntax = ext.into();
    }

    // fn set_theme(theme: &str)

    pub fn start_highlight(&mut self) {
        let snippet = start_highlighted_html_snippet(&self.themes.themes["base16-ocean.dark"]);
        // snippet.0
        self.is_highlighting = true;
    }

    pub fn stop_highlight(&mut self) {
        self.is_highlighting = false;
    }

    pub fn highlight_line(&self, i: &str) -> String {
        let s = self.syntaxes.find_syntax_by_extension(&self.syntax).unwrap();
        let mut h = HighlightLines::new(s, &self.themes.themes["base16-ocean.dark"]);
        let regions = h.highlight(i, &self.syntaxes);
        styled_line_to_highlighted_html(&regions[..], IncludeBackground::No)
    }

    // pub fn highlight<'a>(&self, i: &str) -> String {
    //     format!("{}{}{}", self.start_highlight(), self.highlight_line(i), "")
    // }
}
