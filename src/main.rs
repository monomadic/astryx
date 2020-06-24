mod parse;
mod models;
mod print;

const EXAMPLE: &str = r#"
page path=./index.html title="monomadic"

    row centered
    another centered
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

fn main() {
    match parse::run(EXAMPLE) {
        Ok((r, nodes)) => {
            println!("r: {:?}\n\n", r);
            // println!("r: {:#?}", nodes);
            print::print_nodes(nodes, 0);
        },
        Err(e) => {
            println!("error: {:?}", e);
        },
    }
}
