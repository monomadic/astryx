use std::fmt;
use crate::{filesystem::read_file, error::{AstryxError, AstryxResult}};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) enum Value {
    // Array(Vec<Value>),
    Document(Document),
    Path(String),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Value::Array(a) => write!(f, "{:?}", a),
            Value::Document(doc) => write!(f, "{}", doc.body),
            Value::Path(p) => write!(f, "{:?}", p),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        match value {
            Value::String(s) => s,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Document {
    // created_at: Date
    pub path: PathBuf,
    pub body: String,
    pub metadata: Option<yaml_rust::Yaml>,
}

impl Document {
    pub(crate) fn get(&self, ident: &str) -> Option<String> {
        if ident == "body" {
            return Some(self.body.clone());
        }

        self.metadata
            .clone()
            .map(move |metadata| metadata[ident].clone())
            .and_then(|s| s.as_str().map(|s| s.to_string()))
            .map(|s| s.to_string())
    }

    // TODO rewrite as map
    pub(crate) fn read_from_glob(pattern: &str) -> AstryxResult<Vec<Document>> {
        let options = glob::MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
    
        let mut files = Vec::new();
        let globs = glob::glob_with(pattern, options)
            .map_err(|_| AstryxError::new("error globbing file"))?;
    
        for file in globs {
            // TODO wrap unwrap in error
            let path = file.expect("file to unwrap");
            let filepath: String = path.as_os_str().to_str().unwrap().into();
            let file_content = read_file(filepath)?;
    
            let (yaml, markdown) = crate::frontmatter::parse(&file_content)
                .map_err(|_| AstryxError::new("error reading metadata"))?;
    
            files.push(Document {
                body: crate::markdown::parse(&markdown)?,
                metadata: yaml,
                path,
            });
        }
    
        Ok(files)
    }
}
