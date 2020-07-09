use interpreter::State;

mod error;
mod filesystem;
mod frontmatter;
mod interpolation;
mod interpreter;
mod models;
mod parse;
mod print;

const TARGET_EXAMPLE: &str = r#"
css:
    body { background: red; }

page \
    path="/" \
    title="monomadic"

    row centered
        column max-width="960px" class="main-header"
            image path=./monomadic.svg
            | monomadic

            for post in ./examples/posts/*.md
                | TITLE = ${ post.title }
                link href=post.route
                    | ${ post.title }

                page path=post.route title=post.title
                    | ${ post.title }
                    h1
                        | ${ post.title }
"#;

fn main() {
    match parse::run(TARGET_EXAMPLE) {
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
