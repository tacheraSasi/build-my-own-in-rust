use std::fs;
fn main(){
    noob_approach()
}
fn noob_approach() {
    let paths = fs::read_dir(".").unwrap();
    for path in paths{
        println!("{:?}", path.unwrap().file_name().to_string_lossy());
    }
}
