#[cfg(test)]
mod tests {
    use crate::interpreter;
    use std::fs;

    fn run_file(file_path: String) {
        match fs::read_to_string(file_path) {
            Ok(code_to_execute) => {
                let result = interpreter::execute_interpreter(&code_to_execute);
                println!("{:?}", result);
            }
            Err(e) => {
                panic!("Error reading file: {}", e);
            }
        }
    }

    #[test]
    fn examples_succeed() {
        run_file("./src/examples/hello_world.nali".to_string())
    }
}
