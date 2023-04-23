mod parser;
mod lexer;

fn main() {
    let output = match parser::parse_string(r"Hello A-Ztesting! \s\D \228 \xAF \u2F55 \o") {
        Ok(val) => val,
        Err(e) => panic!("{e}")
    };


}
