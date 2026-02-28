use crate::enums::Token;
use crate::enums::{Expression, Operation};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub expressions: Vec<Expression>,
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

pub fn parse(tokens: &[Token]) -> Program {
    let mut parser = Parser { tokens, pos: 0 };
    let ast = parser.parse_program();

    println!("AST: {:?}", ast);

    if parser.pos != tokens.len() {
        panic!("Has not parsed the entire expression")
    }

    ast
}

impl<'a> Parser<'a> {
    fn parse_program(&mut self) -> Program {
        let mut program = Program {
            expressions: vec![],
        };

        while self.pos < self.tokens.len() {
            let statement = self.parse_statement();
            program.expressions.push(statement);

            match self.tokens.get(self.pos) {
                Some(Token::Semicolon) => self.pos += 1,
                None => break,
                _ => panic!("Expected ';' between statements"),
            }
        }

        program
    }

    fn parse_statement(&mut self) -> Expression {
        match self.tokens.get(self.pos) {
            Some(Token::Remember) => self.parse_declaration(),
            _ => self.parse_assignment(),
        }
    }

    fn parse_declaration(&mut self) -> Expression {
        self.pos += 1; // Consume remember

        let name = match self.tokens.get(self.pos) {
            Some(Token::Identifier(name)) => {
                self.pos += 1;
                name.clone()
            }
            _ => panic!("Expected identifier after remember"),
        };

        if self.tokens.get(self.pos) != Some(&Token::Equals) {
            panic!("Expected '=' after identifier")
        }
        self.pos += 1;

        let value = self.parse_expression();

        Expression::Assign {
            name,
            value: Box::new(value),
        }
    }

    /*
    An assignment follows this pattern:
        identifier -> Equals -> expression
     */
    fn parse_assignment(&mut self) -> Expression {
        let expression = self.parse_expression();

        if let Expression::Variable(ref name) = expression {
            if let Some(Token::Equals) = self.tokens.get(self.pos) {
                self.pos += 1;
                let value = self.parse_assignment();
                return Expression::Assign {
                    name: name.clone(),
                    value: Box::new(value),
                };
            }
        }

        expression
    }

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

        match operation {
            Operation::Add | Operation::Subtract => {
                self.pos += 1;
                Expression::Unary {
                    operation: *operation,
                    expression: Box::new(self.parse_unary()),
                }
            }
            _ => panic!("Invalid token ${:?} at position {}", operation, self.pos),
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
            Some(Token::Identifier(name)) => {
                self.pos += 1;
                Expression::Variable(name.clone())
            }
            _ => panic!("Unexpected token {:?} at position {}", token, self.pos),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_unary_expression() {
        parse(&vec![
            Token::Operation(Operation::Multiply),
            Token::Number(5),
        ]);
    }

    #[test]
    fn assignment() {
        assert_eq!(
            parse(&vec![
                Token::Remember,
                Token::Identifier("test".to_string()),
                Token::Equals,
                Token::Number(15)
            ]),
            Program {
                expressions: vec![Expression::Assign {
                    name: "test".to_string(),
                    value: Box::new(Expression::Number(15))
                }]
            }
        )
    }

    #[test]
    fn multiple_statements() {
        assert_eq!(
            parse(&vec![
                Token::Remember,
                Token::Identifier("test".to_string()),
                Token::Equals,
                Token::Number(15),
                Token::Semicolon,
                Token::Identifier("test".to_string()),
                Token::Operation(Operation::Add),
                Token::Number(5),
            ]),
            Program {
                expressions: vec![
                    Expression::Assign {
                        name: "test".to_string(),
                        value: Box::new(Expression::Number(15))
                    },
                    Expression::Binary {
                        left: Box::new(Expression::Variable("test".to_string())),
                        operation: Operation::Add,
                        right: Box::new(Expression::Number(5)),
                    }
                ]
            }
        )
    }
}
