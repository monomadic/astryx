use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn run() -> Result<(), String> {
    let mut rl = Editor::<()>::new();
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history.");
    // }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line == "quit" || line == "exit" { break };
                rl.add_history_entry(line.as_str());
                eval(&line);
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
    match i {
        _ => println!("{:?}", i)
    }
}
