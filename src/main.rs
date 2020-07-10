use interpreter::State;

mod error;
mod filesystem;
mod frontmatter;
mod interpolation;
mod interpreter;
mod markdown;
mod models;
mod parse;
mod print;
mod server;

fn main() {
    let file = filesystem::read_file(std::path::PathBuf::from("./examples/basic.astryx"))
        .expect("could not read example file");

    server::start(std::path::PathBuf::from("./examples/basic.astryx"), 8888)
        .expect("server crashed");

    // match parse::run(&file) {
    //     Ok((_, nodes)) => {
    //         let state = &mut State::new();
    //         let _ = interpreter::run(&nodes, state).unwrap();
    //         println!("{:#?}", state.page_buffers);
    //     }
    //     Err(e) => {
    //         println!("error: {:?}", e);
    //     }
    // }
}
