use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Project {
    pub pages: HashMap<String, String>,
}

impl Project {
    pub(crate) fn new() -> Self {
        // todo: impl default instead of new
        Project {
            pages: HashMap::new(),
        }
    }

    /// upsert pages as raw html
    pub(crate) fn write_page(&mut self, path: &str, content: &str) {
        if let Some(page) = self.pages.get_mut(path) {
            page.push_str(content);
        } else {
            self.pages.insert(path.into(), content.into());
        }
    }

    // fn push_style()
    // fn push_file()
    // fn push_script()
    // fn export()
}
