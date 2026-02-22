use crate::enums::Token;
use crate::enums::{Expression, Operation};

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

pub fn parse(tokens: &[Token]) -> Expression {
    let mut parser = Parser { tokens, pos: 0 };
    let ast = parser.parse_expression();

    println!("AST: {:?}", ast);

    if parser.pos != tokens.len() {
        panic!("Has not parsed the entire expression")
    }

    ast
}

impl<'a> Parser<'a> {
    fn parse_expression(&mut self) -> Expression {
        // We instantly resolve left
        let mut left = self.parse_term();

        // Iterate over tokens while you still have operations left
        while let Some(Token::Operation(operation)) = self.tokens.get(self.pos) {
            match operation {
                Operation::Add | Operation::Subtract => {
                    self.pos += 1;

                    // We instantly resolve right
                    let right = self.parse_term();

                    left = Expression::Binary {
                        left: Box::new(left),
                        operation: *operation,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        left
    }

    fn parse_term(&mut self) -> Expression {
        // We instantly resolve left
        let mut left = self.parse_unary();

        // Iterate over tokens while you still have operations left
        while let Some(Token::Operation(operation)) = self.tokens.get(self.pos) {
            match operation {
                Operation::Multiply | Operation::Divide => {
                    self.pos += 1;

                    // We instantly resolve right
                    let right = self.parse_unary();

                    left = Expression::Binary {
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

    fn parse_unary(&mut self) -> Expression {
        let Some(Token::Operation(operation)) = self.tokens.get(self.pos) else {
            return self.parse_factor();
        };

        self.pos += 1;
        Expression::Unary {
            operation: *operation,
            expression: Box::new(self.parse_unary()),
        }
    }

    fn parse_factor(&mut self) -> Expression {
        let token = self.tokens.get(self.pos);
        match token {
            Some(Token::Number(n)) => {
                self.pos += 1;
                Expression::Number(*n)
            }
            Some(Token::ParenthesesOpen) => {
                self.pos += 1;
                let expression = self.parse_expression();

                if let Some(Token::ParenthesesClosed) = self.tokens.get(self.pos) {
                    self.pos += 1;
                    expression
                } else {
                    panic!("Expected ')', found {:?}", self.tokens.get(self.pos));
                }
            }
            _ => panic!("Invalid factor {:?}", token),
        }
    }
}
