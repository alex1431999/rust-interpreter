use std::env;

mod cursor;
mod enums;
mod environment;
mod interpreter;
mod parser;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(code_to_execute) = args.get(1) {
        let result = interpreter::execute_interpreter(code_to_execute);
        println!("{:?}", result);
    } else {
        println!("Usage: cargo run -- \"<your code here>\"");
    }
}
