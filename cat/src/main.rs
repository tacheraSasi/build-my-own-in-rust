use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cat <filepath...>");
        return;
    }
    let filepaths = &args[1..];
    
}

fn noob_approach(filepath: &[String]) {
    
}

fn ideal_approach(){}

fn systems_thinking_approach(){}