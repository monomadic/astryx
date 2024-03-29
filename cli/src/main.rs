/// cli interface tool  for the astryx compiler
use astryx::AstryxResult;
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

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
    // /// Start an instance of the Astryx REPL
    // Repl,
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

        /// Server port
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
        #[structopt(parse(from_os_str), default_value = ".")]
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
        Err(e) => println!("{}", e.output()), // todo: print error properly
    }
}

fn run() -> AstryxResult<String> {
    match Opt::from_args().command {
        Command::Serve { input, port } => server::start(&input, port),
        Command::Build {
            input,
            output,
            check,
            stdout,
        } => {
            let t0 = Instant::now();
            println!("parsing {}\n", input.display());

            astryx::parse_from_file(input, None).and_then(|site| {
                if check {
                    println!("read only check. skipping file write...");
                    Ok(())
                } else {
                    if stdout {
                        for (route, page) in site.render_pages() {
                            println!("\n{}:\n{}", route, page);
                        }
                        Ok(())
                    } else {
                        site.write(output)
                    }
                }
            })?;

            println!("\nBuild complete in {:.4}s.", t0.elapsed().as_secs_f64());

            Ok(())
        }
        Command::Init { path } => init::init_project(path),
        // Command::Repl => {
        //     repl::run(); // todo: bubble up error
        //     Ok(())
        // }
    }
    .map(|_| "done.".to_string())
}
