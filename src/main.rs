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
        #[structopt(
            parse(from_os_str),
            short = "i",
            long = "input",
            default_value = "site.astryx"
        )]
        input: PathBuf,
        #[structopt(short = "p", long = "port", default_value = "8888")]
        port: u32,
    },
    /// Build the project into output files
    Build {
        /// Input file
        #[structopt(
            parse(from_os_str),
            short = "i",
            long = "input",
            default_value = "site.astryx"
        )]
        input: PathBuf,
        /// output path
        #[structopt(
            parse(from_os_str),
            short = "o",
            long = "output",
            default_value = "./build"
        )]
        output: PathBuf,
    },
    /// Check the project for errors but do not build anything
    Check {
        /// Input file
        #[structopt(
            parse(from_os_str),
            short = "i",
            long = "input",
            default_value = "site.astryx"
        )]
        input: PathBuf,
    },
    /// Create a new project
    Init {
        /// Init path
        #[structopt(parse(from_os_str), short = "p", long = "path", default_value = ".")]
        path: PathBuf,
    },
}

#[derive(StructOpt, Debug)]
enum BuildOutput {
    Parser,
    Interpreter,
}

pub fn main() {
    match run() {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", display_error(e)),
    }
}

/// run cli commands
fn run() -> AstryxResult<String> {
    match Opt::from_args().command {
        Command::Serve { input, port } => server::start(&input, port),
        Command::Build { input, output } => {
            let file = std::fs::read_to_string(&input)
                .expect(&format!("could not open {}", input.display()));

            println!("building: {}\n", input.display());

            build::build(file, &input)
        }
        Command::Check { input } => {
            let file = std::fs::read_to_string(&input)
                .expect(&format!("could not open {}", input.display()));
            let state = Rc::new(RefCell::new(State::new()));

            println!("checking: {}\n", input.display());

            parser::run(&file, input.to_str().unwrap()) // fixme: remove unwrap
                .map_err(AstryxError::from)
                .and_then(|nodes| interpreter::run(&nodes, state))
                .map(Site::render)
                .map(|_| println!("no errors."))
        }
        Command::Init { path } => init::init_project(),
        Command::Repl => {
            repl::run(); // todo: bubble up error
            Ok(())
        }
    }
    .map(|_| "\ndone.".to_string())
}
