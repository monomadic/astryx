use interpreter::State;

mod error;
mod interpreter;
mod models;
mod parse;
mod print;
mod filesystem;

const TARGET_EXAMPLE: &str = r#"
css:
    body { background: red; }

page \
    path=./index.html \
    title="monomadic"

    row centered
        column max-width="960px" class="main-header"
            image path=./monomadic.svg
            | monomadic

            for post in ./examples/posts/*.md
                link href=post
                    | post.title

                page path=post.path
                    h1
                        | ${ post.title }
                    | ${ post.body }
"#;

fn main() {
    match parse::run(TARGET_EXAMPLE) {
        Ok((_, nodes)) => {
            let state = &mut State::new();
            // println!("{:?}", nodes);
            print::print_nodes(nodes.clone(), 0);

            let _ = interpreter::run(&nodes, state).unwrap();
            println!("{:#?}", state.page_buffers);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
}
