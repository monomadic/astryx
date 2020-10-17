use rustyline::error::ReadlineError;
use rustyline::Editor;
use interpreter::State;

pub fn run() -> Result<(), String> {
    let mut rl = Editor::<()>::new();
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history.");
    // }

    let state = &mut State::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                // ast dump (start line with :)
                if line.chars().collect::<Vec<char>>()[0] == ':' {
                    println!("ast: {:?}", parser::run(&crop_letters(&line, 1)))
                }

                // command (start line with .)
                if line.chars().collect::<Vec<char>>()[0] == '.' {
                    match line.as_str() {
                        ".quit" | ".exit" | ".q" => break,
                        ".state" | ".s" => println!("state: {:?}", state),
                        _ => println!("no such command: {}", line),
                    }
                } else {
                    println!("{:?}", parser::run(&line)
                        .map(|s| interpreter::run(s, state)))
                }
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
    // rl.save_history("history.txt").unwrap();

    Ok(())
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
