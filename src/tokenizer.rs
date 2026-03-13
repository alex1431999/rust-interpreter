use crate::cursor::Cursor;
use crate::enums::{Comparator, Operation, Token};

struct Tokenizer<'a> {
    characters: &'a [char],
    tokens: Vec<Token>,
    position: usize,
}

pub fn tokenize(code_to_execute: &str) -> Vec<Token> {
    let characters: Vec<char> = code_to_execute.chars().collect();
    let tokenizer = Tokenizer {
        characters: &characters,
        tokens: vec![],
        position: 0,
    };

    tokenizer.tokenize()
}

impl Cursor<char> for Tokenizer<'_> {
    fn items(&self) -> &[char] {
        self.characters
    }

    fn position(&self) -> usize {
        self.position
    }

    fn position_mut(&mut self) -> &mut usize {
        &mut self.position
    }
}

impl<'a> Tokenizer<'a> {
    fn tokenize(mut self) -> Vec<Token> {
        while self.has_next() {
            if self.process_white_space() {
                continue;
            }

            if self.process_string() {
                continue;
            }

            if self.process_number() {
                continue;
            }

            if self.process_equality() {
                continue;
            }

            if self.process_identifier() {
                continue;
            }

            if self.process_basic_tokens() {
                continue;
            }

            panic!(
                "Unexpected character '{}' at {}",
                self.get_current(),
                self.position
            )
        }

        self.tokens
    }

    fn process_white_space(&mut self) -> bool {
        let character = self.get_current();

        if character.is_whitespace() {
            self.advance(1);
            return true;
        }

        false
    }

    fn process_string(&mut self) -> bool {
        let mut character = self.get_current();

        if character == '"' {
            self.tokens.push(Token::Quote);
            self.advance(1);

            let mut string_value = String::new();

            character = self.get_current();
            while character != '"' {
                string_value.push(character);
                self.advance(1);
                character = self.get_current();
            }

            self.advance(1);

            self.tokens.push(Token::String(string_value));
            self.tokens.push(Token::Quote);

            true
        } else {
            false
        }
    }

    fn process_number(&mut self) -> bool {
        let character = self.get_current();

        if character.is_ascii_digit() {
            let mut number = 0;
            while self.has_next() && self.get_current().is_ascii_digit() {
                number = number * 10 + self.get_current().to_digit(10).unwrap() as i64;
                self.advance(1)
            }
            self.tokens.push(Token::Number(number));
            return true;
        }

        false
    }

    fn process_identifier(&mut self) -> bool {
        let character = self.get_current();
        if is_identifier_character(character, true) {
            let mut identifier = String::new();
            while self.has_next()
                && is_identifier_character(self.get_current(), identifier.is_empty())
            {
                identifier.push(self.get_current());
                self.advance(1)
            }

            let token = match identifier.as_str() {
                "remember" => Token::Remember,
                "yell" => Token::Yell,
                "true" => Token::True,
                "false" => Token::False,
                "if" => Token::If,
                "else" => Token::Else,
                "while" => Token::While,
                "null" => Token::Null,
                _ => Token::Identifier(identifier),
            };
            self.tokens.push(token);

            return true;
        }

        false
    }

    fn process_equality(&mut self) -> bool {
        if self.items_left() < 2 {
            return false;
        }

        let character = self.get_current();
        let next_character = self.get_next();

        if character == '=' && next_character == '=' {
            self.tokens.push(Token::Comparator(Comparator::Equality));
            self.advance(2);
            true
        } else {
            false
        }
    }

    fn process_basic_tokens(&mut self) -> bool {
        let character = self.get_current();

        match character {
            '+' => self.tokens.push(Token::Operation(Operation::Add)),
            '-' => self.tokens.push(Token::Operation(Operation::Subtract)),
            '*' => self.tokens.push(Token::Operation(Operation::Multiply)),
            '/' => self.tokens.push(Token::Operation(Operation::Divide)),
            '(' => self.tokens.push(Token::ParenthesesOpen),
            ')' => self.tokens.push(Token::ParenthesesClosed),
            '=' => self.tokens.push(Token::Equals),
            ';' => self.tokens.push(Token::Semicolon),
            '{' => self.tokens.push(Token::BlockOpen),
            '}' => self.tokens.push(Token::BlockClosed),
            '<' => self.tokens.push(Token::Comparator(Comparator::LessThan)),
            '>' => self.tokens.push(Token::Comparator(Comparator::GreaterThan)),
            '"' => self.tokens.push(Token::Quote),
            '[' => self.tokens.push(Token::BracketOpen),
            ']' => self.tokens.push(Token::BracketClosed),
            ',' => self.tokens.push(Token::Comma),
            _ => return false,
        }

        self.advance(1);

        true
    }
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
            tokenize("remember test_123 = 5"),
            vec![
                Token::Remember,
                Token::Identifier("test_123".parse().unwrap()),
                Token::Equals,
                Token::Number(5)
            ]
        )
    }

    #[test]
    fn true_and_false() {
        assert_eq!(tokenize("true false"), vec![Token::True, Token::False])
    }

    #[test]
    fn if_support() {
        assert_eq!(tokenize("if"), vec![Token::If])
    }

    #[test]
    fn equality_support() {
        assert_eq!(
            tokenize("=="),
            vec![Token::Comparator(Comparator::Equality)]
        )
    }
}
