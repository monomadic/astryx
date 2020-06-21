mod parse;
mod models;
mod print;

const EE: &str = r#"
page path=./index.html title="hello"
    row centered
"#;

const EXAMPLE: &str = r#"
page path=./index.html title="hello"
    row one
    row two

"#;

fn main() {
    let (r, nodes) = parse::run(EXAMPLE).unwrap();
    println!("{}\n", EXAMPLE);
    print::print_nodes(nodes, 0);
    // println!("{:?}", result);
}
