// cli tool for the astryx compiler

use astryx::{AstryxError, AstryxResult, Site, State};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use structopt::StructOpt;

mod init;
// mod server;

#[derive(StructOpt, Debug)]
#[structopt(name = "astryx")]
struct Opt {
    /// Command
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    // /// Start an instance of the Astryx REPL
    // Repl,
    /// Start a server for the current project
    // Serve {
    //     /// Input file
    //     #[structopt(
    //         parse(from_os_str),
    //         short = "i",
    //         long = "input",
    //         default_value = "site.astryx"
    //     )]
    //     input: PathBuf,
    //
    //     /// Server port
    //     #[structopt(short = "p", long = "port", default_value = "8888")]
    //     port: u32,
    // },
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

        /// Perform a read-only check (don't write files)
        #[structopt(long = "check")]
        check: bool,

        /// Write the file output to stdout instead of files
        #[structopt(long = "stdout")]
        stdout: bool,
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
        Err(e) => println!("{:?}", e), // todo: print error properly
    }
}

fn run() -> AstryxResult<String> {
    match Opt::from_args().command {
        // Command::Serve { input, port } => server::start(&input, port),
        Command::Build {
            input,
            output,
            check,
            stdout,
        } => {
            println!("building: {}\n", input.display());

            astryx::parse_from_file(input, None).map(|site| {
                if check {
                    println!("read only check. skipping file write...")
                } else {
                    if stdout {
                        for (route, document) in site.documents {
                            println!("{}: {}", route, document);
                        }
                    } else {
                        site.write()
                    }
                }
            })
        }
        Command::Init { path } => init::init_project(),
        // Command::Repl => {
        //     repl::run(); // todo: bubble up error
        //     Ok(())
        // }
    }
    .map(|_| "\ndone.".to_string())
}
