use error::AstryxResult;
use std::path::PathBuf;
use structopt::StructOpt;

mod error;
mod filesystem;
mod frontmatter;
mod highlighter;
mod html;
mod interpolation;
mod interpreter;
mod markdown;
mod models;
mod parse;
mod print;
mod server;

#[derive(StructOpt, Debug)]
#[structopt(name = "cassette")]
struct Opt {
    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    match run() {
        Ok(_) => println!("\n"),
        Err(e) => println!("\n\nERROR: {:?}", e),
    }
}

pub fn run() -> AstryxResult<()> {
    let opt = Opt::from_args();

    server::start(opt.file, 8888)
}
