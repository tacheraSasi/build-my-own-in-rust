use std::fs;

fn main() {
    let paths = fs::read_dir(".").unwrap();
    for path in paths{
        println!("{:?}", path.unwrap().file_name().to_string_lossy());
    }
}
