use std::fs;

fn main() {
    let code_to_execute = fs::read_to_string("./src/example.nali").unwrap();
    println!("{}", code_to_execute);
}
