use crate::enums::{Operation, Token};

pub fn tokenize(code_to_execute: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let characters: Vec<char> = code_to_execute.chars().collect();

    let mut number_being_parsed: i64 = 0;
    let mut identifier_being_parsed = String::new();

    for i in 0..characters.len() {
        let character = characters[i];
        let has_next_character = (i + 1) < characters.len();

        if is_identifier_character(character, identifier_being_parsed.is_empty()) {
            identifier_being_parsed.push(character);

            if !has_next_character || !is_identifier_character(characters[i + 1], false) {
                tokens.push(Token::Identifier(identifier_being_parsed.clone()));
                identifier_being_parsed = String::new();
            }

            continue;
        }

        if character.is_ascii_digit() {
            number_being_parsed = number_being_parsed * 10 + character.to_digit(10).unwrap() as i64;

            if !has_next_character || !characters[i + 1].is_ascii_digit() {
                tokens.push(Token::Number(number_being_parsed));
                number_being_parsed = 0;
            }

            continue;
        }

        match character {
            '+' => tokens.push(Token::Operation(Operation::Add)),
            '-' => tokens.push(Token::Operation(Operation::Subtract)),
            '*' => tokens.push(Token::Operation(Operation::Multiply)),
            '/' => tokens.push(Token::Operation(Operation::Divide)),
            '(' => tokens.push(Token::ParenthesesOpen),
            ')' => tokens.push(Token::ParenthesesClosed),
            ' ' => {} // We just ignore white space for now
            _ => panic!("Unexpected character {}, at position {}", character, i),
        }
    }

    tokens
}

fn is_identifier_character(character: char, is_first_character: bool) -> bool {
    if is_first_character {
        character.is_ascii_alphabetic() || character == '_'
    } else {
        character.is_ascii_alphanumeric() || character == '_'
    }
}

#[cfg(test)]
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

    #[test]
    fn mixed_spacing_and_multi_digit() {
        assert_eq!(
            tokenize("  12+  34   *5 "),
            vec![
                Token::Number(12),
                Token::Operation(Operation::Add),
                Token::Number(34),
                Token::Operation(Operation::Multiply),
                Token::Number(5),
            ]
        )
    }

    #[test]
    fn parentheses() {
        assert_eq!(
            tokenize("12 * (5 + 5)"),
            vec![
                Token::Number(12),
                Token::Operation(Operation::Multiply),
                Token::ParenthesesOpen,
                Token::Number(5),
                Token::Operation(Operation::Add),
                Token::Number(5),
                Token::ParenthesesClosed,
            ]
        )
    }

    #[test]
    fn identifier() {
        assert_eq!(
            tokenize("test_123"),
            vec![Token::Identifier("test_123".parse().unwrap())]
        )
    }
}
