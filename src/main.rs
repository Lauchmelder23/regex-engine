extern crate regex;

use std::{env};

use regex::regex::Regex;

fn print_usage() {
    println!("Usage: ./regex-cli <regexp>");
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    
    if args.len() != 2 {
        print_usage();
        return;
    }
    
    let regexp_str = &args[1];
    let regex = match Regex::new(regexp_str) {
        Err(err) => { eprintln!("{}", err); return; },
        Ok(res) => res
    };

    dbg!(regex.test("abbb"));
}
