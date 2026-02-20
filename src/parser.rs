use crate::enums::Token;
use crate::enums::{Expr, Operation};

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

pub fn parse(tokens: &[Token]) -> Expr {
    let mut parser = Parser { tokens, pos: 0 };
    let expression = parser.parse_expression();

    if parser.pos != tokens.len() {
        panic!("Has not parsed the entire expression")
    }

    expression
}

impl<'a> Parser<'a> {
    fn parse_expression(&mut self) -> Expr {
        // We instantly resolve left
        let mut left = self.parse_term();

        // Iterate over tokens while you still have operations left
        while let Some(Token::Operation(operation)) = self.tokens.get(self.pos) {
            match operation {
                Operation::Add | Operation::Subtract => {
                    self.pos += 1;

                    // We instantly resolve right
                    let right = self.parse_term();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operation: *operation,
                        right: Box::new(right),
                    }
                }
                _ => break,
            }
        }

        left
    }

    fn parse_term(&mut self) -> Expr {
        // We instantly resolve left
        let mut left = self.parse_factor();

        // Iterate over tokens while you still have operations left
        while let Some(Token::Operation(operation)) = self.tokens.get(self.pos) {
            match operation {
                Operation::Multiply => {
                    self.pos += 1;

                    // We instantly resolve right
                    let right = self.parse_factor();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operation: *operation,
                        right: Box::new(right),
                    }
                }
                _ => break,
            }
        }

        left
    }

    fn parse_factor(&mut self) -> Expr {
        let token = self.tokens.get(self.pos);
        match token {
            Some(Token::Number(n)) => {
                self.pos += 1;
                Expr::Number(*n)
            }
            _ => panic!("Invalid factor {:?}", token),
        }
    }
}
