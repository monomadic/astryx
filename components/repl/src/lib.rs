use interpreter::State;
use rustyline::{error::ReadlineError, Editor};
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() -> Result<(), String> {
    let mut editor = Editor::<()>::new();

    repl(Rc::new(RefCell::new(State::default())), &mut editor);
    Ok(())
}

fn repl(state: Rc<RefCell<State>>, editor: &mut Editor<()>) {
    print_logo();
    loop {
        match editor.readline(">> ") {
            Ok(line) => {
                let inner = Rc::clone(&state);
                let statements = parser::run(&line);
                println!("parser: {:?}", statements);

                match interpreter::run(&statements.unwrap(), inner) {
                    Ok(_) => {}
                    Err(e) => println!("error: {:?}", e),
                };

                // println!("state: {:?}", statements);
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

    // repl(state, editor);
}

fn print_logo() {
    println!(
        "{}",
        r#"
        _____/\\\\\\\\\_________/\\\\\\\\\\\_____/\\\\\\\\\\\\\\\_____/\\\\\\\\\_______/\\\________/\\\___/\\\_______/\\\_        
        ___/\\\\\\\\\\\\\_____/\\\/////////\\\__\///////\\\/////____/\\\///////\\\____\///\\\____/\\\/___\///\\\___/\\\/__       
         __/\\\/////////\\\___\//\\\______\///_________\/\\\________\/\\\_____\/\\\______\///\\\/\\\/_______\///\\\\\\/____      
          _\/\\\_______\/\\\____\////\\\________________\/\\\________\/\\\\\\\\\\\/_________\///\\\/___________\//\\\\______     
           _\/\\\\\\\\\\\\\\\_______\////\\\_____________\/\\\________\/\\\//////\\\___________\/\\\_____________\/\\\\______    
            _\/\\\/////////\\\__________\////\\\__________\/\\\________\/\\\____\//\\\__________\/\\\_____________/\\\\\\_____   
             _\/\\\_______\/\\\___/\\\______\//\\\_________\/\\\________\/\\\_____\//\\\_________\/\\\___________/\\\////\\\___  
              _\/\\\_______\/\\\__\///\\\\\\\\\\\/__________\/\\\________\/\\\______\//\\\________\/\\\_________/\\\/___\///\\\_ 
               _\///________\///_____\///////////____________\///_________\///________\///_________\///_________\///_______\///__
"#
    );
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
