

/// returns a nom combinator version of the parser
pub fn run(i: &str) -> IResult<&str, Metadata> {
    let (mut r, (pre, _, _, ident, _, _, _, relative_path, _)) = trim(nom::sequence::tuple((
        multispace0,
        tag("for"),
        space1,
        alphanumeric1,
        space0,
        tag("in"),
        space1,
        relative_path,
        blank_lines,
    )))(i)?;
}

fn yaml_frontmatter(i: &str) -> IResult<&str, FrontMatter> {
    nom::sequence::tuple(multispace0, tag("---"), yaml_values, tag("---"), markdown)
}

fn yaml_values(i: &str) -> IResult<&str, FrontMatter> {
}
