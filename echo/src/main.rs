use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // noob_approach(args);
    ideal_approach();
}

fn noob_approach(args: Vec<String>) {
    for i in 1..args.len() {
        print!("{} ", args[i]);
    }
    println!();
}

fn ideal_approach() {
    let output = env::args().skip(1).collect::<Vec<String>>().join(" ");
    println!("{}", output);
}
