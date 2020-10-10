use crate::error::{AstryxError, AstryxResult};
use parser::Statement;

pub trait RenderErrorAsHTML {
    fn to_html(&self) -> String;
}

impl RenderErrorAsHTML for AstryxError<'_> {
    fn to_html(&self) -> String {
        String::from("hi")
    }
}

pub fn render<'a>(file: &'a str) -> AstryxResult<'a, std::collections::HashMap<String, String>> {
    // let file: &'a str = &std::fs::read_to_string(&path).expect("file");

    parser::run(file)
        .map_err(AstryxError::from)
        .and_then(|ast: Vec<Statement>| interpreter::run(&ast).map_err(AstryxError::from))
        .and_then(|_html_nodes| {
            html::render_as_string(&std::collections::HashMap::new()) // FIXME this will totally break everything rn
                .map_err(AstryxError::from)
        })
        .map_err(AstryxError::from)
}
