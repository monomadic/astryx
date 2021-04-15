use error::{display::display_error, AstryxError, AstryxResult};
use models::{Site, State};
use repl;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use structopt::StructOpt;

mod build;
mod init;
mod server;

#[derive(StructOpt, Debug)]
#[structopt(name = "astryx")]
struct Opt {
    /// Command
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Repl,
    /// start a server
    Serve {
        /// Input file
        file: Option<String>,
        port: Option<u32>,
    },
    /// build the project
    Build {
        /// Input file
        input: Option<String>,
        output: Option<String>,
    },
    Check {
        /// Input file
        file: Option<String>,
    },
    Init {
        /// Init path
        path: Option<PathBuf>,
    },
}

pub fn main() {
    match run() {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e),
    }
}

/// run cli commands
fn run() -> Result<String, String> {
    let opt = Opt::from_args();

    // todo: match arm should be an AstryxError until last minute for string conversion
    match opt.command {
        Command::Serve { file, port } => {
            let path = &file.unwrap_or(String::from("site.astryx"));

            server::start(path.into(), port.unwrap_or(8888)).map_err(|e| display_error(&e, path))
        }
        Command::Build { input, output } => {
            let path = &input.unwrap_or(String::from("site.astryx"));
            let file = std::fs::read_to_string(&path).expect(&format!("could not open {}", path));

            println!("building: {}\n", &path);
            build::build(&file, &path).map_err(|e| display_error(&e, path))
        }
        Command::Check { file } => {
            let path = &file.unwrap_or(String::from("site.astryx"));
            let file = std::fs::read_to_string(&path).expect(&format!("could not open {}", path));

            println!("checking: {}\n", &path);
            let state = Rc::new(RefCell::new(State::new()));

            parser::run(&file, path)
                .map_err(AstryxError::from)
                .and_then(|nodes| interpreter::run(&nodes, state))
                .map(Site::render)
                .map(|_| println!("no errors."))
                .map_err(|e| display_error(&e, path))
        }
        Command::Init { path } => {
            init::init_project().map_err(|e| format!("error creating new project: {:?}", e))
        }
        Command::Repl => {
            repl::run(); // todo: bubble up error
            Ok(())
        }
    }
    .map(|_| "\ndone.".to_string())
}
