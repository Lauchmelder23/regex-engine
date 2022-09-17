mod parser;
mod node;

use std::{env};

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
    let mut regex = match parser::parse("Ba(na*)+") {
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
    dbg!(regex.test("Banaaaaanaaanaaana"));
    dbg!(regex.test("Banannnnaan"));
}
