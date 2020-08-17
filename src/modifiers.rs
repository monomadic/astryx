// collection of imported functions into state

use crate::{error::AstryxResult, html::HTMLElement};

#[derive(Debug, Clone)]
pub(crate) struct Imports {
    // types: HashMap<String, Type>,
// modifiers: HashMap<String, Modifier>,
}

impl Imports {
    pub(crate) fn new() -> Self {
        // let mut types = HashMap::new();
        // types.insert("row", )

        // Imports {
        //     types: HashMap::new(),
        //     modifiers: HashMap::new(),
        // }
        Self {}
    }

    pub(crate) fn create_element(&self, ident: &str) -> AstryxResult<HTMLElement> {
        match ident {
            "row" | "column" | "grid" => Ok(HTMLElement::new_with_class("div", ident)),
            _ => HTMLElement::new_from_html_tag(ident),
        }
    }

    pub(crate) fn modify_element(
        &self,
        modifier: &str,
        args: Option<&String>,
        el: &mut HTMLElement,
    ) -> AstryxResult<()> {
        match modifier.into() {
            // check for the ident type
            "align.left" => {
                // el.styles.push(format!("grid-template-columns: {};", s));
                el.add_class("align-left");
            }
            "align.right" => {
                // el.styles.push(format!("grid-template-columns: {};", s));
                el.add_class("align-right");
            }
            "background-color" => el.add_style(format!(
                "background-color: {};",
                args.unwrap_or(&String::new())
            )),
            _ => panic!("cannot find modifier {}", modifier),
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Modifier {
    processor: fn(HTMLElement) -> AstryxResult<HTMLElement>,
}
#[derive(Debug, Clone)]
struct Type {
    // todo: extra checking eg. require certain modifiers, defaults, etc.
    element_ident: String,
    modifiers: Vec<Modifier>,
    // required: Vec<String>,
}
