mod parse;
mod models;

const EXAMPLE: &str = r#"
page path=./ title="hello"
"#;

const EEE: &str = r#"
"hello"
"hi"
page centered title="hello" path=./index.html
"#;

fn main() {
    let result = parse::run(EEE).unwrap();
    println!("{}", EEE);
    println!("{:?}", result);
}
