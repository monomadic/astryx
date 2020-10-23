use interpreter::{State, Writer};
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn run() -> Result<(), String> {
    let mut rl = Editor::<()>::new();
    let mut state = State::new();
    state.writer = Writer::StdOut;
    repl(&mut rl, state);
    Ok(())
}

fn repl(editor: &mut Editor<()>, state: State) {
    if let Some(line) = read_line(editor, state.clone()) {
        let temp_state = &mut state.clone();

        match parser::run(&line) {
            Ok(statements) => {
                match interpreter::run(statements, temp_state) {
                    Ok(_) => {}
                    Err(e) => println!("interpreter error: {:?}", e),
                };
            }
            Err(e) => println!("parser error: {:?}", e),
        }

        let new_state = temp_state.clone();
        repl(editor, new_state);
    }
}

fn read_line(rl: &mut Editor<()>, state: State) -> Option<String> {
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
                        ".state" | ".s" => println!("state: {:?}", state),
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
