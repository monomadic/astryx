use interpreter::State;

mod error;
mod functions;
mod interpreter;
mod models;
mod parse;
mod print;

const TARGET_EXAMPLE: &str = r#"
page \
    path=./index.html \
    title="monomadic"

    row centered
        column max-width="960px" class="main-header"
            image path=./monomadic.svg
            | monomadic
            for post in ./posts
                link href=post
                    | post.title
                page post.path
                    h1
                        | ${ post.title }
                        | ${ post.body }
"#;

// const EXAMPLE: &str = r#"
// page path=./index.html
// "#;

fn main() {
    match parse::run(TARGET_EXAMPLE) {
        Ok((_, nodes)) => {
            // interpret syntax
            let r = interpreter::run(&nodes, &mut State::new()).unwrap();
            // print::print_nodes(r, 0);
        },
        Err(e) => {
            println!("error: {:?}", e);
        },
    }
}
