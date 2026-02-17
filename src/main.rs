mod interpreter;

use std::fs;

fn main() {
    let code_to_execute = fs::read_to_string("./src/example.nali").unwrap();
    interpreter::execute(code_to_execute)
}
