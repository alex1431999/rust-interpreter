use crate::enums::{Operation, Token};

struct Tokenizer<'a> {
    characters: &'a [char],
    tokens: Vec<Token>,
    pos: usize,
}

pub fn tokenize(code_to_execute: &str) -> Vec<Token> {
    let characters: Vec<char> = code_to_execute.chars().collect();
    let tokenizer = Tokenizer {
        characters: &characters,
        tokens: vec![],
        pos: 0,
    };

    tokenizer.tokenize()
}

impl<'a> Tokenizer<'a> {
    fn tokenize(mut self) -> Vec<Token> {
        while self.has_more() {
            if self.process_white_space() {
                continue;
            }

            if self.process_number() {
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
                self.get_current_character(),
                self.pos
            )
        }

        self.tokens
    }

    fn process_white_space(&mut self) -> bool {
        let character = self.get_current_character();

        if character.is_whitespace() {
            self.advance(1);
            return true;
        }

        false
    }

    fn process_number(&mut self) -> bool {
        let character = self.get_current_character();

        if character.is_ascii_digit() {
            let mut number = 0;
            while self.has_more() && self.get_current_character().is_ascii_digit() {
                number = number * 10 + self.get_current_character().to_digit(10).unwrap() as i64;
                self.advance(1)
            }
            self.tokens.push(Token::Number(number));
            return true;
        }

        false
    }

    fn process_identifier(&mut self) -> bool {
        let character = self.get_current_character();
        if is_identifier_character(character, true) {
            let mut identifier = String::new();
            while self.has_more()
                && is_identifier_character(self.get_current_character(), identifier.is_empty())
            {
                identifier.push(self.get_current_character());
                self.advance(1)
            }

            let token = match identifier.as_str() {
                "remember" => Token::Remember,
                "yell" => Token::Yell,
                "true" => Token::True,
                "false" => Token::False,
                _ => Token::Identifier(identifier),
            };
            self.tokens.push(token);

            return true;
        }

        false
    }

    fn process_basic_tokens(&mut self) -> bool {
        let character = self.get_current_character();

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
            _ => return false,
        }

        self.advance(1);

        true
    }

    fn get_current_character(&self) -> char {
        self.characters[self.pos]
    }

    fn has_more(&self) -> bool {
        self.pos < self.characters.len()
    }

    fn advance(&mut self, amount: usize) {
        self.pos += amount
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
}
