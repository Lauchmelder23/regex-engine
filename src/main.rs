mod parser;
mod lexer;

fn main() {
    let output = match parser::parse_string("Hello A-Ztesting!") {
        Ok(val) => val,
        Err(e) => panic!("{e}")
    };


}
