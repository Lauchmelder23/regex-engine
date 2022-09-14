mod regex;
mod elements;

use std::env;
use crate::regex::Regex;

fn print_usage() {
    println!("Usage: ./regex <regexp>");
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    
    if args.len() != 2 {
        print_usage();
        return;
    }
    
    let regexp_str = &args[1];
    let regex = Regex::new(regexp_str);

    dbg!(regex.test("criiiing"));
}
