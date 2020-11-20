use interpreter::State;
use rustyline::Editor;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() -> Result<(), String> {
    let mut editor = Editor::<()>::new();

    repl(Rc::new(RefCell::new(State::default())), &mut editor);
    Ok(())
}

fn repl(state: Rc<RefCell<State<'_>>>, editor: &mut Editor<()>) {
    let line: String = editor.readline(">> ").unwrap();

    let inner = Rc::clone(&state);
    // let statements = parser::run(&line).unwrap();
    // let _ = interpreter::run(&statements, inner).unwrap();
    // println!("{:?}", statements);
    // repl(inner, editor);
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
