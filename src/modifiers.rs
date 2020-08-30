
pub(crate) enum ModifierArgs {
    Simple,
    AnonArgs(Vec<Value>),
    NamedArgs(HashMap<String, Value>),
}

// note that .title and #title are aliases for the functions add-class("title") and set-id("title").
// the parenthesis can be omitted in the syntax so width.expand and width.expand() are equivalent.

pub(crate) struct ModifierLibrary {

}

impl ModifierLibrary {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn get(ident: &str, args: ModifierArgs) -> AstryxResult<Fn(&mut HTMLElement)> {

    }
}

struct Modifier {
    valid_tags: Vec<String>,
    // required_args: ?
    do_update: Fn(&mut HTMLElement) -> AstryxResult<HTMLElement>
}
