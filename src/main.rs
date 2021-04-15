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
    /// Start an instance of the Astryx REPL
    Repl,
    /// Start a server for the current project
    Serve {
        /// Input file
        #[structopt(parse(from_os_str))]
        file: Option<PathBuf>,
        port: Option<u32>,
    },
    /// Build the project into output files
    Build {
        /// Input file
        #[structopt(parse(from_os_str))]
        input: Option<PathBuf>,
        output: Option<String>,
    },
    /// Check the project for errors but do not build anything
    Check {
        /// Input file
        #[structopt(parse(from_os_str))]
        file: Option<PathBuf>,
    },
    /// Create a new project
    Init {
        /// Init path
        #[structopt(parse(from_os_str))]
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
    // todo: match arm should be an AstryxError until last minute for string conversion
    match Opt::from_args().command {
        Command::Serve { file, port } => {
            let path = file.unwrap_or(PathBuf::from("site.astryx"));
            let port = port.unwrap_or(8888);

            server::start(&path, port).map_err(display_error)
        }
        Command::Build { input, output } => {
            let path = input.unwrap_or(PathBuf::from("site.astryx"));
            let file = std::fs::read_to_string(&path)
                .expect(&format!("could not open {}", path.display()));

            println!("building: {}\n", path.display());

            build::build(file, &path).map_err(display_error)
        }
        Command::Check { file } => {
            let path = file.unwrap_or(PathBuf::from("site.astryx"));
            let file = std::fs::read_to_string(&path)
                .expect(&format!("could not open {}", path.display()));
            let state = Rc::new(RefCell::new(State::new()));

            println!("checking: {}\n", path.display());

            parser::run(&file, path.to_str().unwrap()) // fixme: remove unwrap
                .map_err(AstryxError::from)
                .and_then(|nodes| interpreter::run(&nodes, state))
                .map(Site::render)
                .map(|_| println!("no errors."))
                .map_err(display_error)
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
