#[derive(Debug, Clone)]
pub enum Variable {
    RelativePath(String),
    QuotedString(String),
    Reference(String),
    TemplateFile(TemplateFile),
    // FunctionCall()
}

// caution: does not resolve references.
impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::RelativePath(s) | Variable::QuotedString(s) | Variable::Reference(s) => {
                f.write_str(s)
            }
            Variable::TemplateFile(t) => f.write_str(&t.body),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemplateFile {
    // created_at: Date
    pub body: String,
    // pub filename: String,
    // pub variables: HashMap<String, String>,
    pub metadata: Option<yaml_rust::Yaml>,
}

// /// Converts a series of variables to strings
// pub(crate) fn stringify_variables(
//     variables: &HashMap<String, Variable>,
//     locals: &HashMap<String, Variable>,
// ) -> AstryxResult<HashMap<String, String>> {
//     let mut stringified: HashMap<String, String> = HashMap::new();

//     for (ident, variable) in variables {
//         stringified.insert(
//             ident.clone(),
//             stringify_variable(variable, locals)?,
//         );
//     }

//     Ok(stringified)
// }

// TODO scope should not bleed from interpreter, ever.
// pub(crate) fn stringify_variable(
//     variable: &Variable,
//     locals: &HashMap<String, Variable>,
// ) -> Result<String, ParserError> {
//     match variable {
//         Variable::RelativePath(p) => Ok(p.clone()),
//         Variable::Reference(p) => {
//             // FIXME unsafe array index
//             // if let Some(ref lang) = info.split('.').next() {
//             let base_ref: &str = p.split(".").collect::<Vec<&str>>()[0];
//             let subref: &str = p.split(".").collect::<Vec<&str>>()[1];

//             if let Some(Variable::TemplateFile(template_file)) = locals.get(base_ref) {
//                 if subref == "body" {
//                     Ok(template_file.body.clone())
//                 } else {
//                     let yaml_var = template_file.metadata.clone().unwrap()[subref].clone();

//                     match yaml_var {
//                         Yaml::String(s) => Ok(s),
//                         _ => Err(ParserError::new(&format!("reference_not_found: {}", subref))),
//                     }
//                 }
//             } else {
//                 locals
//                     .get(p)
//                     .ok_or(ParserError::new(&format!(
//                         "reference_not_found: {} {:?}",
//                         &p, &locals
//                     )))
//                     .and_then(|v| stringify_variable(v, locals))
//             }
//         }
//         Variable::QuotedString(p) => Ok(p.clone()),
//         Variable::TemplateFile(t) => {
//             // a page object has been printed directly. use its body.
//             Ok(t.body.clone())
//         }
//     }
// }
