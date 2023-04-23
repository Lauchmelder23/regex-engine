mod parser;
mod lexer;

fn main() {
    let output = match parser::parse_string(r"[^A-Za-z]") {
        Ok(val) => val,
        Err(e) => panic!("{e}")
    };


}
