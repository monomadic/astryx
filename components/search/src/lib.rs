use elasticlunr::{Index, Language};

pub const ELASTICLUNR_JS: &str = include_str!("elasticlunr.min.js");

pub struct IndexBuilder {
    index: Index
}

impl IndexBuilder {
    pub fn new() -> Self {
        IndexBuilder {
            index: Index::with_language(Language::English, &["title", "body"])
        }
    }

    pub fn insert(&mut self, title: &str, body: &[&str]) {
        self.index.add_doc(title, body);
    }

    pub fn to_json(&self) -> String {
        String::new()
    }
}
