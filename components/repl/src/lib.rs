use interpreter::{builtins, State};
use rustyline::{error::ReadlineError, Editor};
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    let state = Rc::new(RefCell::new(State::default()));
    let inner = &builtins::import(state);
    repl(Rc::clone(inner), &mut Editor::<()>::new());
}

fn repl(state: Rc<RefCell<State>>, editor: &mut Editor<()>) {
    print_logo();
    loop {
        match editor.readline(">> ") {
            Ok(line) => {
                if line == ".state" {
                    println!("{:?}", state.borrow().local);
                    continue;
                }

                if line.chars().collect::<Vec<char>>()[0] == ':' {
                    println!("{:?}", parser::run(&pop_chars(&line, 1)));
                    continue;
                }

                match parser::run(&line) {
                    Ok(statements) => {
                        for statement in statements {
                            match interpreter::eval(statement.borrow().clone(), Rc::clone(&state)) {
                                Ok(object) => println!("=> {}", object.borrow().inspect()),
                                Err(e) => println!("interpreter error: {:?}", e),
                            }
                        }
                    }
                    Err(e) => println!("parser error: {:?}", e),
                };
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

/// pops `pos` characters from the front of a string, returning remainder
fn pop_chars(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
