use crate::enums::Token::Number;
use crate::enums::{Operation, Token};

pub fn tokenize(code_to_execute: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let characters: Vec<char> = code_to_execute.chars().collect();

    let mut number_being_parsed: i64 = 0;

    for i in 0..characters.len() {
        let character = characters[i];
        let has_next_character = (i + 1) < characters.len();

        if character.is_digit(10) {
            number_being_parsed = number_being_parsed * 10 + character.to_digit(10).unwrap() as i64;

            if !has_next_character || !characters[i + 1].is_digit(10) {
                tokens.push(Number(number_being_parsed));
                number_being_parsed = 0;
            }

            continue;
        }

        match character {
            '+' => tokens.push(Token::Operation(Operation::Add)),
            '-' => tokens.push(Token::Operation(Operation::Subtract)),
            '*' => tokens.push(Token::Operation(Operation::Multiply)),
            '/' => tokens.push(Token::Operation(Operation::Divide)),
            ' ' => {} // We just ignore white space for now
            _ => panic!("Unsupported character {}", character),
        }
    }

    tokens
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

    #[test]
    fn no_white_space() {
        assert_eq!(
            tokenize("55+55"),
            vec![
                Token::Number(55),
                Token::Operation(Operation::Add),
                Token::Number(55)
            ]
        )
    }
}
