
use crate::models::Attribute;


// pub(crate) fn render_attributes(attributes: &Vec<Attribute>) -> String {
//     attributes.iter().filter_map(|a| {
//         match a {
//             Attribute::NamedAttribute{ ident, variable} => {
//                 Some(format!(" {}=\"{}\"", ident, variable))
//             }
//             // Attribute::Decorator(d) => {

//             // }
//             _ => None
//         }
//     }).collect()
// }

// pub fn html_img(buffer: &str, src: &str)

// pub fn html_tag(ident: &str, attributes: Vec<(String, String)>, content: &str) -> String {
// }

// pub fn html_tag(ident: &str, attributes: Vec<(String, String)>) -> String {
//     let attribs = if !attributes.is_empty() {
//         format!(
//             " {}",
//             attributes
//                 .iter()
//                 .map(|(k, v)| format!("{}=\"{}\"", k, v))
//                 .collect::<Vec<String>>()
//                 .join(" ")
//         )
//     } else {
//         String::new()
//     };

//     format!("<{}{}>", ident, attribs)
// }
