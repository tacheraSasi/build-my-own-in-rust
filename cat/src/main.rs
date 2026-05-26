use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cat <filepath...>");
        return;
    }
    let filepaths = &args[1..];
    noob_approach(filepaths);
}

fn noob_approach(filepaths: &[String]) {
    for path in filepaths {
        let content = fs::read_to_string(path).unwrap_or_default();
        println!("{}", content);
    }
}

fn ideal_approach(){}

fn systems_thinking_approach(){}