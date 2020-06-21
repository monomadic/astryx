mod parse;
mod models;
mod print;

const EXAMPLE: &str = r#"
page path=./index.html title="hello"
    row centered
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
        Ok((_, nodes)) => {
            print::print_nodes(nodes, 0);
        },
        Err(e) => println!("error: {:?}", e),
    }
}
