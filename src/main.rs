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
            link href=post.href
                | post.title
            for post in ./posts
                link href=post
                page post.path
                    h1
                        | ${ post.title }
                        | ${ post.body }
"#;

const EE: &str = r#"
page
  one
  two
  three
    four
  five

gumby
  princess
  bumbum
"#;

fn main() {
    match parse::run(EXAMPLE) {
        Ok((r, nodes)) => {
            println!("r: {:?}\n\n", r);
            println!("r: {:#?}", nodes);
            print::print_nodes(nodes, 0);
        },
        Err(e) => {
            println!("error: {:?}", e);
        },
    }
}
