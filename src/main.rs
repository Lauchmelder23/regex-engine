extern crate regex;

use std::{env};

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
    let mut regex = match regex::new("Ba(na)+na") {
        Err(err) => { eprintln!("{}", err); return; },
        Ok(res) => res
    };

    // regex = dbg!(regex);
    // println!("/{}/", regexp_str);
    // dbg!(regex.test("criiiinge"));
    // dbg!(regex.test("based"));
    // dbg!(regex.test("crooioionge"));
    // dbg!(regex.test("crnge"));
    dbg!(regex.test("Ba"));
    dbg!(regex.test("Banana"));
    dbg!(regex.test("Banananananana"));
    dbg!(regex.test("Banannnnaan"));
}
