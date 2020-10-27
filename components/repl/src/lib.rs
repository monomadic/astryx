use interpreter::{State, Writer};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() -> Result<(), String> {
    let mut rl = Editor::<()>::new();
    let mut state = State::default();
    state.writer = Writer::StdOut;

    let state = Rc::new(RefCell::new(state));

    //
    repl(&mut rl, state);
    Ok(())
}

fn repl<'a>(editor: &mut Editor<()>, state: Rc<RefCell<State<'a>>>) {
    let line = editor.readline(">> ");

    // let response = read_line(line, Rc::clone(&state)).unwrap();

    let response = "let a = 2".to_string();

    editor.add_history_entry(response);

    let local = Rc::new(RefCell::new(State::extend(state)));

    // let statements = parser::run(&response).unwrap();
    // let result = interpreter::run(&statements, local).unwrap();

    let mut state = State::new();
    state.writer = Writer::StdOut;

    let state = Rc::new(RefCell::new(state));
    // state.writer = Writer::File("index.html".to_string());
    // state.writer = Writer::StdOut;

    // parser::run(&response).map(|nodes| interpreter::run(&nodes, state));

    // let statements = parser::run(&line).unwrap();
    // interpreter::run(statements, Rc::clone(&state));

    // if let Some(line) = read_line(editor, Rc::clone(&state)) {

    //     // match parser::run(&line) {
    //     //     Ok(statements) => {
    //     //         // for statement in statements {
    //     //         //     println!("{}", statement.borrow().inspect());
    //     //         //     interpreter::run(vec![statement], Rc::clone(&state));
    //     //         // }
    //     //         match interpreter::run(statements, Rc::clone(&state)) {
    //     //             Ok(_) => repl(editor, Rc::clone(&state)),
    //     //             Err(e) => println!("interpreter error: {:?}", e),
    //     //         };
    //     //     }
    //     //     Err(e) => println!("parser error: {:?}", e),
    //     // }
    // }
}

fn read_line(readline: Result<String, ReadlineError>, state: Rc<RefCell<State>>) -> Option<String> {
    loop {
        match readline {
            Ok(line) => {
                // ast dump (start line with :)
                if line.chars().collect::<Vec<char>>()[0] == ':' {
                    println!("{:?}", parser::run(&crop_letters(&line, 1)));
                    continue;
                }

                // command (start line with .)
                if line.chars().collect::<Vec<char>>()[0] == '.' {
                    match line.as_str() {
                        ".quit" | ".exit" | ".q" => break,
                        ".state" | ".s" => println!("state"),
                        _ => println!("no such command: {}", line),
                    }
                    continue;
                }

                // println!("{}", state.borrow_mut().local.inspect());
                return Some(line);
                // return None;
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("error: {:?}", err);
                break;
            }
        }
    }

    None
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
