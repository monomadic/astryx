mod parse;
mod models;

const EXAMPLE: &str = r#"
page path=./ title="hello"
"#;

const EEE: &str = r#"
"hello"
"hi"
page path=./ title="hello"
"#;

fn main() {
    let result = parse::run(EEE).unwrap();
    println!("{:?}", result);
}
