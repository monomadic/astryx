use yaml_rust::{scanner::ScanError, Yaml, YamlLoader};

pub(crate) fn parse(text: &str) -> Result<(Option<Yaml>, String), ScanError> {
    let (yaml, content) = parse_and_find_content(text)?;
    Ok((yaml, String::from(content)))
}

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

/// read a yaml frontmatter block at the start of any document
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
