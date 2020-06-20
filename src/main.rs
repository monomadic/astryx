mod parse;
mod models;

const EXAMPLE: &str = r#"
page path=./index.html title="hello"
    row centered
"#;

fn main() {
    let result = parse::run(EXAMPLE).unwrap();
    println!("{}", EXAMPLE);
    println!("{:?}", result);
}
