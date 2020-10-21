use rustyline::error::ReadlineError;
use rustyline::Editor;
use interpreter::State;

pub fn run() -> Result<(), String> {
    let mut rl = Editor::<()>::new();
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history.");
    // }

    let state = &mut State::new();

    repl(&mut rl, State::new());

    // loop {
    //     let readline = rl.readline(">> ");
    //     match readline {
    //         Ok(line) => {
    //             rl.add_history_entry(line.as_str());

    //             // ast dump (start line with :)
    //             if line.chars().collect::<Vec<char>>()[0] == ':' {
    //                 println!("ast: {:?}", parser::run(&crop_letters(&line, 1)));
    //                 continue
    //             }

    //             // command (start line with .)
    //             if line.chars().collect::<Vec<char>>()[0] == '.' {
    //                 match line.as_str() {
    //                     ".quit" | ".exit" | ".q" => break,
    //                     ".state" | ".s" => println!("state: {:?}", state),
    //                     _ => println!("no such command: {}", line),
    //                 }
    //                 continue
    //             }

    //             let statements = parser::run(&line);
    //             println!("{:?}", &statements);

    //             let result = interpreter::run(statements.unwrap(), state);

    //             // let statements = parser::run(&line)
    //             //     .map_err(AstryxError::from)
    //             //     .and_then(|nodes| interpreter::run(nodes, state).map_err(AstryxError::from));
    //             // let nodes = statements.map(|s| interpreter::run(s, state));

    //             // build::build(&file).map_err(|e| display_error(&e, path))

    //             // println!("{:?}", &statements);
    //         },
    //         Err(ReadlineError::Interrupted) => {
    //             println!("CTRL-C");
    //             break
    //         },
    //         Err(ReadlineError::Eof) => {
    //             println!("CTRL-D");
    //             break
    //         },
    //         Err(err) => {
    //             println!("error: {:?}", err);
    //             break
    //         }
    //     }
    // }
    // // rl.save_history("history.txt").unwrap();

    Ok(())
}

fn repl(editor: &mut Editor<()>, state: State) {
    // let state = &mut State::new();

    let line = read_line(editor, state.clone());

    if let Some(line) = read_line(editor, state) {
        let statements = parser::run(&line);
        println!("{:?}", &statements);

        let temp_state = &mut state.clone();
        let _ = interpreter::run(statements.unwrap(), temp_state);

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
                    println!("ast: {:?}", parser::run(&crop_letters(&line, 1)));
                    continue
                }

                // command (start line with .)
                if line.chars().collect::<Vec<char>>()[0] == '.' {
                    match line.as_str() {
                        ".quit" | ".exit" | ".q" => break,
                        ".state" | ".s" => println!("state: {:?}", state),
                        _ => println!("no such command: {}", line),
                    }
                    continue
                }

                return Some(line);

                // let statements = parser::run(&line);
                // println!("{:?}", &statements);

                // let result = interpreter::run(statements.unwrap(), state);

                // let statements = parser::run(&line)
                //     .map_err(AstryxError::from)
                //     .and_then(|nodes| interpreter::run(nodes, state).map_err(AstryxError::from));
                // let nodes = statements.map(|s| interpreter::run(s, state));

                // build::build(&file).map_err(|e| display_error(&e, path))

                // println!("{:?}", &statements);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("error: {:?}", err);
                break
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
