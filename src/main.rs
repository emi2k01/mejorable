mod iter;
mod parser;
mod scanner;
mod span;
mod token;

fn main() {
    let src = r#""#;

    for line_input in src.split('\n') {
        let tokens = scanner::line::LineScanner::new(line_input.chars()).scan();
        println!("{:#?}", tokens);
    }
}
