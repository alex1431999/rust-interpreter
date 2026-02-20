mod enums;
mod interpreter;
mod parser;
mod tokenizer;

use std::fs;

fn main() {
    let code_to_execute = fs::read_to_string("./src/example.nali").unwrap();

    let result = interpreter::execute_interpreter(code_to_execute.as_str());

    println!("{}", result)
}
