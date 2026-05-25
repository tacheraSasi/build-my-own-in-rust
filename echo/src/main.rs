use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    noob_approach(args);
}

fn noob_approach(args: Vec<String>) {
    for i in 1..args.len() {
        print!("{} ", args[i]);
    }
    println!();
}

// fn ideal_approach(args: Vec<String>) {}
