use std::{env, fs};
use std::io::{self, Read};


fn main() {
    let args: Vec<String> = env::args().collect();
    // noob_approach(args);
    ideal_approach(args);
    // systems_thinking_approach();
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

fn ideal_approach(args: Vec<String>) {
    if args.len() == 1 {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        println!("{}", buffer);
    } else {
        let filepaths = &args[1..];
        for path in filepaths {
            let content = fs::read_to_string(path).unwrap();
            println!("{}", content);
        }
    }
}

fn systems_thinking_approach() {}
