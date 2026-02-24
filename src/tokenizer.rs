use crate::enums::{Operation, Token};

pub fn tokenize(code_to_execute: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let characters: Vec<char> = code_to_execute.chars().collect();
    let mut i = 0;

    while i < characters.len() {
        let character = characters[i];

        if character.is_whitespace() {
            i += 1;
            continue;
        }

        if character.is_ascii_digit() {
            let mut number = 0;
            while i < characters.len() && characters[i].is_ascii_digit() {
                number = number * 10 + characters[i].to_digit(10).unwrap() as i64;
                i += 1;
            }
            tokens.push(Token::Number(number));
            continue;
        }

        if is_identifier_character(character, true) {
            let mut ident = String::new();
            while i < characters.len() && is_identifier_character(characters[i], ident.is_empty()) {
                ident.push(characters[i]);
                i += 1;
            }
            tokens.push(Token::Identifier(ident));
            continue;
        }

        match character {
            '+' => tokens.push(Token::Operation(Operation::Add)),
            '-' => tokens.push(Token::Operation(Operation::Subtract)),
            '*' => tokens.push(Token::Operation(Operation::Multiply)),
            '/' => tokens.push(Token::Operation(Operation::Divide)),
            '(' => tokens.push(Token::ParenthesesOpen),
            ')' => tokens.push(Token::ParenthesesClosed),
            '=' => tokens.push(Token::Equals),
            _ => panic!("Unexpected character '{}' at {}", character, i),
        }

        i += 1;
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
            tokenize("test_123 = 5"),
            vec![
                Token::Identifier("test_123".parse().unwrap()),
                Token::Equals,
                Token::Number(5)
            ]
        )
    }
}
