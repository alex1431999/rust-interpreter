mod enums;
mod interpreter;
mod parser;
mod tokenizer;

use std::fs;

fn main() {
    let code_to_execute = "5 + 5";

    let result = interpreter::execute_interpreter(code_to_execute);

    println!("{:?}", result)
}
