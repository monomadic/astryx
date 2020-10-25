struct SymbolTableStack {
    local: HashMap<String, Object>,
    outer: Option<SymbolTableStack>,
}

struct SymbolTable;

impl SymbolTable {
    fn get(ident: &str) -> Option<Symbol> {}
}

struct Symbol {}
