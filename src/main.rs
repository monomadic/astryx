use std::path::PathBuf;
use structopt::StructOpt;
use astryx::{self, error::AstryxResult, };

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
