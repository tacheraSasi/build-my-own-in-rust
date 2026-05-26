use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cat <filepaths...>");
        return;
    }
    noob_approach(args);
}

fn noob_approach(args: Vec<String>) {
    if args.len() < 2 {
        println!("Usage: cat <filepaths...>");
        return;
    }
    let filepaths = &args[1..];
    for path in filepaths {
        let content = fs::read_to_string(path).unwrap();
        println!("{}", content);
    }
}

fn ideal_approach(){}

fn systems_thinking_approach(){}