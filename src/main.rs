mod parser;
//mod tokenizer;
mod token;
mod span;
mod line;
mod iter;

fn main() {
    let src = r#""#;

    for line_input in src.split('\n') {
        let tokens = line::scanner::LineScanner::new(line_input.chars()).scan();
        println!("{:#?}", tokens);
    }
}
