pub use yaml_rust::Yaml;
use yaml_rust::{scanner::ScanError, YamlLoader};

// TODO consolidate these two functions
/// parses a file and returns it as a string with optional separated yaml frontmatter
pub fn parse(text: &str) -> Result<(Option<Yaml>, String), ScanError> {
    let (yaml, content) = parse_and_find_content(text)?;
    Ok((yaml, String::from(content)))
}

/// parses a file and returns it as a string with optional separated yaml frontmatter
fn parse_and_find_content(text: &str) -> Result<(Option<Yaml>, &str), ScanError> {
    match find_yaml_block(text) {
        Some((fm_start, fm_end, content_start)) => {
            let yaml_str = &text[fm_start..fm_end];
            let mut documents = YamlLoader::load_from_str(yaml_str)?;
            let rest_of_text = &text[content_start..];

            Ok((documents.pop(), rest_of_text))
        }
        None => Ok((None, text)),
    }
}

// TODO support other frontmatter syntaxes
/// return the location (start, end, content start) of yaml frontmatter inside a document
fn find_yaml_block(text: &str) -> Option<(usize, usize, usize)> {
    match text.starts_with("---\n") {
        true => {
            let slice_after_marker = &text[4..];
            let fm_end = slice_after_marker.find("---\n");
            if fm_end.is_none() {
                return None;
            };

            let fm_end = fm_end.unwrap();
            Some((4, fm_end + 4, fm_end + 2 * 4))
        }
        false => None,
    }
}
