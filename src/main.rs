mod parser;
mod regex;
mod elements;

use std::env;

fn print_usage() {
    println!("Usage: ./regex-engine <regexp>");
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    
    if args.len() != 2 {
        print_usage();
        return;
    }
    
    let regexp_str = &args[1];
    let regex = parser::parse(regexp_str);

    dbg!(regex.test("cringe"));
}
