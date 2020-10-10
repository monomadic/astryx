
type LocalData = HashMap<String, Value>;
// TODO make private type
#[derive(Debug, Clone)]
pub(crate) struct State {
    locals: LocalData,
    // pub(crate) pages: Layouts,
    // pub(crate) imports: Imports,
    // pub(crate) pwd: String,
}

impl State {
    pub(crate) fn new() -> Self {
        State {
            locals: LocalData::new(),
        }
    }
}
