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
    let file = filesystem::read_file(std::path::PathBuf::from("./examples/basic.astryx")).expect("could not read example file");
    match parse::run(&file) {
        Ok((_, nodes)) => {
            let state = &mut State::new();
            // print::print_nodes(nodes.clone(), 0);

            let _ = interpreter::run(&nodes, state).unwrap();
            println!("{:#?}", state.page_buffers);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
}
