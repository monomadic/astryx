// collection of imported functions into state

use crate::{
    error::{AstryxError, AstryxResult},
    html::HTMLElement,
};

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

    // TODO remove
    pub(crate) fn create_element(&self, ident: &str) -> AstryxResult<HTMLElement> {
        match ident {
            // "row" | "column" | "grid" => Ok(HTMLElement::new_with_class("div", ident)),
            _ => HTMLElement::new_from_html_tag(ident),
        }
    }

    pub(crate) fn modify_element(
        &self,
        modifier: &str,
        args: Option<&String>,
        el: &mut HTMLElement,
    ) -> AstryxResult<()> {

        if let Some(args) = args {
            el.attributes.insert(modifier.into(), args.into());
        } else {
            panic!("modify elementcalled {}", modifier);
        }

        // match &*el.ident { // fix this
        //     "a" => {
        //         match modifier.into() {
        //             "path" => {
        //                 el.attributes.insert("href".into(), args.unwrap().into());
        //                 return Ok(());
        //             }
        //             _ => ()
        //         }
        //     }
        //     _ => {}
        // }

        // match modifier.into() {
        //     "layout.vertical" => {
        //         el.add_class("layout-vertical");
        //     }
        //     "layout.horizontal" => {
        //         el.add_class("layout-horizontal");
        //     }
        //     "layout.columns" => {
        //         el.add_class("layout-columns");
        //     }
        //     "layout.rows" => {
        //         el.add_class("layout-rows");
        //     }
        //     // check for the ident type
        //     "align.left" => {
        //         // el.styles.push(format!("grid-template-columns: {};", s));
        //         el.add_class("align-left");
        //     }
        //     "align.right" => {
        //         // el.styles.push(format!("grid-template-columns: {};", s));
        //         el.add_class("align-right");
        //     }
        //     "width.full" => {
        //         el.add_style("width: 100%;");
        //     }
        //     "background-color" => el.add_style(format!(
        //         "background-color: {};",
        //         args.unwrap_or(&String::new())
        //     )),
        //     "text.align.center" => el.add_style("text-align: center;"),
        //     // "padding" => el.add_style(format!(
        //     //     "background-color: {};",
        //     //     args.unwrap_or(&String::new())
        //     // )),
        //     // _ => { return Err(AstryxError::new(&format!("cannot find modifier {}", modifier)))},
        //     _ => {
        //         el.attributes.insert(modifier.into(), args.unwrap().into());
        //         return Ok(());
        //     }
        // }

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
