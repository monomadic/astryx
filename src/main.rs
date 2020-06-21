mod parse;
mod models;
mod print;

const EXAMPLE: &str = r#"
page
    path=./index.html
    title="monomadic"

    row centered
    another centered
        column max-width="960px" class="main-header"
            image path=./monomadic.svg
            | monomadic
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
            println!("r: {:?}", r);
            println!("r: {:#?}", nodes);
            print::print_nodes(nodes, 0);
        },
        Err(e) => {
            println!("error: {:?}", e);
        },
    }
}
