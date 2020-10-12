use structopt::StructOpt;
use error::AstryxResult;

mod server;
mod render;
mod build;
mod error;

#[derive(StructOpt, Debug)]
#[structopt(name = "astryx")]
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
        file: Option<String>,
        port: Option<u32>,
    },
    /// build the project
    Build {
        /// Input file
        file: Option<String>,
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
fn run<'a>() -> AstryxResult<'a, ()> {
    let opt = Opt::from_args();

    match opt.command {
        Command::Serve{ file, port } =>
            server::start(
                file.unwrap_or(String::from("site.astryx")),
                port.unwrap_or(8888)),
        Command::Build{ file } => {
            let file = file.unwrap_or(String::from("site.astryx"));
            println!("building: {}\n", &file);
            build::build(
            &file)
        },
        Command::New => new_project(),
    }
}

/// set up a new project in the current directory
fn new_project<'a>() -> AstryxResult<'a, ()> {
    Ok(())
}
