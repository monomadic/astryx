use std::path::PathBuf;
use structopt::StructOpt;
use astryx::{self, error::AstryxResult, };

mod server;

#[derive(StructOpt, Debug)]
#[structopt(name = "cassette")]
struct Opt {
    /// Command
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// start a server
    Serve {
        /// Input file
        #[structopt(parse(from_os_str))]
        file: PathBuf, // TODO make optional (default site.astryx)
        port: Option<u32>,
    },
    /// build the project
    Build {
        /// Input file
        #[structopt(parse(from_os_str))]
        file: PathBuf, // TODO make optional (default site.astryx)
    },
    New,
}

pub fn main() {
    match run() {
        Ok(_) => println!("\n"),
        Err(e) => println!("\n\nERROR: {:?}", e),
    }
}

/// run cli commands
fn run() -> AstryxResult<()> {
    let opt = Opt::from_args();

    match opt.command {
        Command::Serve{ file, port } => server::start(file, port.unwrap_or(8888)),
        Command::Build{ .. } => Ok(()),
        Command::New => new_project(),
    }
}

/// set up a new project in the current directory
fn new_project() -> AstryxResult<()> {
    Ok(())
}
