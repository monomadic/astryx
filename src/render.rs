use astryx::error::AstryxError;

pub trait RenderErrorAsHTML {
    fn to_html(&self) -> String;
}

impl RenderErrorAsHTML for AstryxError {
    fn to_html(&self) -> String {
        String::from("hi")
    }
}
