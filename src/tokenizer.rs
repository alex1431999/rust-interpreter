use crate::enums::{Operation, Token};

pub fn tokenize(code_to_execute: &str) -> Vec<Token> {
    let words: Vec<&str> = code_to_execute.split_whitespace().collect();
    words
        .iter()
        .map(|word| match word.parse::<i64>() {
            Ok(n) => Token::Number(n),
            Err(_) if *word == "+" => Token::Operation(Operation::Add),
            Err(_) if *word == "-" => Token::Operation(Operation::Subtract),
            Err(_) if *word == "*" => Token::Operation(Operation::Multiply),
            Err(_) if *word == "/" => Token::Operation(Operation::Divide),
            Err(_) => panic!("Invalid syntax {}", word),
        })
        .collect()
}

mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(
            tokenize("5 + 5"),
            vec![
                Token::Number(5),
                Token::Operation(Operation::Add),
                Token::Number(5)
            ]
        )
    }
}
