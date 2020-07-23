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

    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf, // TODO make optional (default site.astryx)
}

#[derive(StructOpt, Debug)]
enum Command {
    /// start a server
    Serve,
    /// build the project
    Build,
}

fn main() {
    match run() {
        Ok(_) => println!("\n"),
        Err(e) => println!("\n\nERROR: {:?}", e),
    }
}

pub fn run() -> AstryxResult<()> {
    let opt = Opt::from_args();

    match opt.command {
        Command::Serve => server::start(opt.file, 8888),
        Command::Build => Ok(()),
    }
}
