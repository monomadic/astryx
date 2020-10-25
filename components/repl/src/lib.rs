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
    if let Some(line) = read_line(editor, Rc::clone(&state)) {
        match parser::run(&line) {
            Ok(statements) => {
                // match interpreter::run(statements, Rc::clone(&state)) {
                //     Ok(_) => {}
                //     Err(e) => println!("interpreter error: {:?}", e),
                // };
            }
            Err(e) => println!("parser error: {:?}", e),
        }

        repl(editor, Rc::clone(&state));
    }
}

fn read_line(rl: &mut Editor<()>, state: Rc<RefCell<State>>) -> Option<String> {
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

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

                return Some(line);
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
