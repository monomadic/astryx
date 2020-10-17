use rustyline::error::ReadlineError;
use rustyline::Editor;
use interpreter::State;

pub fn run() -> Result<(), String> {
    let mut rl = Editor::<()>::new();
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history.");
    // }

    let mut state = State::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                match line.as_str() {
                    ".quit" | ".exit" | ".q" => break,
                    ".state" | ".s" => println!("State: {:?}", state),
                    _ => eval(&line),
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
                println!("Error: {:?}", err);
                break
            }
        }
    }
    // rl.save_history("history.txt").unwrap();

    Ok(())
}

fn eval(i: &str) {
    let result = parser::run(i);
    // match i {
    //     _ => println!("{:?}", i)
    // }
    println!("{:?}", result);
}
