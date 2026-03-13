use crate::cursor::Cursor;
use crate::enums::Token;
use crate::enums::{Expression, Operation};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub expressions: Vec<Expression>,
}

struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl Cursor<Token> for Parser<'_> {
    fn items(&self) -> &[Token] {
        self.tokens
    }

    fn position(&self) -> usize {
        self.position
    }

    fn position_mut(&mut self) -> &mut usize {
        &mut self.position
    }
}

pub fn parse(tokens: &[Token]) -> Program {
    let mut parser = Parser {
        tokens,
        position: 0,
    };
    let ast = parser.parse_program();

    if parser.position != tokens.len() {
        panic!("Has not parsed the entire expression")
    }

    ast
}

impl<'a> Parser<'a> {
    fn parse_program(&mut self) -> Program {
        let mut program = Program {
            expressions: vec![],
        };

        while self.position < self.tokens.len() {
            let statement = self.parse_statement();
            program.expressions.push(statement);

            match self.tokens.get(self.position) {
                Some(Token::Semicolon) => self.advance(1),
                None => break,
                _ => panic!("Expected ';' between statements"),
            }
        }

        program
    }

    fn parse_statement(&mut self) -> Expression {
        match self.tokens.get(self.position) {
            Some(Token::Remember) => self.parse_declaration(),
            Some(Token::Yell) => self.parse_yell(),
            Some(Token::If) => self.parse_if(),
            Some(Token::While) => self.parse_while(),
            Some(Token::For) => self.parse_for(),
            _ => self.parse_assignment(),
        }
    }

    fn parse_declaration(&mut self) -> Expression {
        self.consume(&Token::Remember); // Consume remember

        let name = match self.tokens.get(self.position) {
            Some(Token::Identifier(name)) => {
                self.advance(1);
                name.clone()
            }
            _ => panic!("Expected identifier after remember"),
        };

        self.consume(&Token::Equals);

        let value = self.parse_expression();

        Expression::Assign {
            name,
            value: Box::new(value),
        }
    }

    fn parse_yell(&mut self) -> Expression {
        self.consume(&Token::Yell);
        self.consume(&Token::ParenthesesOpen);
        let expression = self.parse_expression();
        self.consume(&Token::ParenthesesClosed);

        Expression::Yell {
            expression: Box::new(expression),
        }
    }

    fn parse_if(&mut self) -> Expression {
        self.consume(&Token::If);

        self.consume(&Token::ParenthesesOpen);
        let condition = self.parse_comparator();
        self.consume(&Token::ParenthesesClosed);

        let success_expression = self.parse_block();

        if self.tokens.get(self.position) == Some(&Token::Else) {
            self.consume(&Token::Else);

            let failure_expression = self.parse_block();

            Expression::If {
                condition: Box::new(condition),
                success_expression: Box::new(success_expression),
                failure_expression: Some(Box::new(failure_expression)),
            }
        } else {
            Expression::If {
                condition: Box::new(condition),
                success_expression: Box::new(success_expression),
                failure_expression: None,
            }
        }
    }

    fn parse_for(&mut self) -> Expression {
        self.consume(&Token::For);
        self.consume(&Token::ParenthesesOpen);

        let name = match self.get_current() {
            Token::Identifier(identifier) => identifier,
            _ => panic!(
                "Invalid token ${:?} at position {}",
                self.get_current(),
                self.position
            ),
        };
        self.consume(&Token::Identifier(name.clone()));
        self.consume(&Token::In);

        let list = self.parse_factor();

        self.consume(&Token::ParenthesesClosed);

        let expression = self.parse_expression();

        Expression::For {
            identifier: name,
            list: Box::new(list),
            expression: Box::new(expression),
        }
    }

    fn parse_while(&mut self) -> Expression {
        self.consume(&Token::While);
        self.consume(&Token::ParenthesesOpen);
        let condition = self.parse_comparator();
        self.consume(&Token::ParenthesesClosed);

        let expression = self.parse_block();

        Expression::While {
            condition: Box::new(condition),
            expression: Box::new(expression),
        }
    }

    /*
    An assignment follows this pattern:
        identifier -> Equals -> expression
     */
    fn parse_assignment(&mut self) -> Expression {
        let expression = self.parse_comparator();

        if let Expression::Variable(ref name) = expression {
            if let Some(Token::Equals) = self.tokens.get(self.position) {
                self.advance(1);
                let value = self.parse_assignment();
                return Expression::Assign {
                    name: name.clone(),
                    value: Box::new(value),
                };
            }
        }

        expression
    }

    fn parse_comparator(&mut self) -> Expression {
        let left = self.parse_expression();

        if let Some(Token::Comparator(comparator)) = self.tokens.get(self.position) {
            self.advance(1);
            let right = self.parse_expression();

            Expression::Comparison {
                left: Box::new(left),
                comparator: *comparator,
                right: Box::new(right),
            }
        } else {
            left
        }
    }

    fn parse_expression(&mut self) -> Expression {
        // We instantly resolve left
        let mut left = self.parse_term();

        // Iterate over tokens while you still have operations left
        while let Some(Token::Operation(operation)) = self.tokens.get(self.position) {
            match operation {
                Operation::Add | Operation::Subtract => {
                    self.advance(1);

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
        while let Some(Token::Operation(operation)) = self.tokens.get(self.position) {
            match operation {
                Operation::Multiply | Operation::Divide => {
                    self.advance(1);

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
        let Some(Token::Operation(operation)) = self.tokens.get(self.position) else {
            return self.parse_factor();
        };

        match operation {
            Operation::Add | Operation::Subtract => {
                self.advance(1);
                Expression::Unary {
                    operation: *operation,
                    expression: Box::new(self.parse_unary()),
                }
            }
            _ => panic!(
                "Invalid token ${:?} at position {}",
                operation, self.position
            ),
        }
    }

    fn parse_factor(&mut self) -> Expression {
        let token = self.tokens.get(self.position);
        match token {
            Some(Token::Number(n)) => {
                self.advance(1);
                Expression::Number(*n)
            }
            Some(Token::True) => {
                self.advance(1);
                Expression::Boolean(true)
            }
            Some(Token::False) => {
                self.advance(1);
                Expression::Boolean(false)
            }
            Some(Token::Null) => {
                self.advance(1);
                Expression::Null
            }
            Some(Token::ParenthesesOpen) => {
                self.advance(1);
                let expression = self.parse_expression();
                self.consume(&Token::ParenthesesClosed);

                expression
            }
            Some(Token::Identifier(name)) => {
                self.advance(1);
                Expression::Variable(name.clone())
            }
            Some(Token::BlockOpen) => self.parse_block(),
            Some(Token::Quote) => {
                self.advance(1);

                match (
                    self.tokens[self.position].clone(),
                    self.tokens[self.position + 1].clone(),
                ) {
                    (Token::String(string_value), Token::Quote) => {
                        self.advance(2);
                        Expression::String(string_value)
                    }
                    _ => panic!("Unexpected token {:?} at position {}", token, self.position),
                }
            }
            Some(Token::BracketOpen) => {
                self.advance(1);

                let mut items = vec![];

                while self.get_current() != Token::BracketClosed {
                    let item = self.parse_statement();
                    items.push(item);

                    let current = self.get_current();
                    match current {
                        Token::BracketClosed => break,
                        Token::Comma => self.consume(&Token::Comma),
                        _ => panic!(
                            "Unexpected token {:?} at position {}",
                            current, self.position
                        ),
                    }
                }

                self.consume(&Token::BracketClosed);

                Expression::List(items)
            }
            _ => panic!("Unexpected token {:?} at position {}", token, self.position),
        }
    }

    fn parse_block(&mut self) -> Expression {
        self.consume(&Token::BlockOpen);

        let mut expressions: Vec<Expression> = vec![];

        while self.get_current() != Token::BlockClosed {
            let expression = self.parse_statement();
            expressions.push(expression);

            match self.tokens.get(self.position) {
                Some(Token::Semicolon) => self.advance(1),
                Some(Token::BlockClosed) => break,
                _ => panic!("Expected ';' or '}}' in block"),
            }
        }

        self.consume(&Token::BlockClosed);

        Expression::Block { expressions }
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

    #[test]
    fn yell() {
        assert_eq!(
            parse(&vec![
                Token::Yell,
                Token::ParenthesesOpen,
                Token::Number(5),
                Token::ParenthesesClosed,
            ]),
            Program {
                expressions: vec![Expression::Yell {
                    expression: Box::new(Expression::Number(5))
                },]
            }
        )
    }

    #[test]
    fn block() {
        assert_eq!(
            parse(&vec![
                Token::BlockOpen,
                Token::Number(5),
                Token::Operation(Operation::Add),
                Token::Number(5),
                Token::Semicolon,
                Token::BlockClosed,
            ]),
            Program {
                expressions: vec![Expression::Block {
                    expressions: vec![Expression::Binary {
                        left: Box::new(Expression::Number(5)),
                        operation: Operation::Add,
                        right: Box::new(Expression::Number(5)),
                    }]
                }]
            }
        )
    }

    #[test]
    fn if_statement() {
        assert_eq!(
            parse(&vec![
                Token::If,
                Token::ParenthesesOpen,
                Token::True,
                Token::ParenthesesClosed,
                Token::BlockOpen,
                Token::Number(5),
                Token::Operation(Operation::Add),
                Token::Number(5),
                Token::Semicolon,
                Token::BlockClosed,
                Token::Else,
                Token::BlockOpen,
                Token::Number(10),
                Token::Operation(Operation::Add),
                Token::Number(10),
                Token::Semicolon,
                Token::BlockClosed,
            ]),
            Program {
                expressions: vec![Expression::If {
                    condition: Box::new(Expression::Boolean(true)),
                    success_expression: Box::new(Expression::Block {
                        expressions: vec![Expression::Binary {
                            left: Box::new(Expression::Number(5)),
                            operation: Operation::Add,
                            right: Box::new(Expression::Number(5)),
                        }]
                    },),
                    failure_expression: Some(Box::new(Expression::Block {
                        expressions: vec![Expression::Binary {
                            left: Box::new(Expression::Number(10)),
                            operation: Operation::Add,
                            right: Box::new(Expression::Number(10)),
                        }]
                    },))
                }]
            }
        )
    }

    #[test]
    #[should_panic]
    fn if_statement_invalid() {
        // The if statement is missing a block
        parse(&vec![
            Token::If,
            Token::ParenthesesOpen,
            Token::True,
            Token::ParenthesesClosed,
        ]);
    }

    #[test]
    fn string() {
        assert_eq!(
            parse(&vec![
                Token::Quote,
                Token::String("test".to_string()),
                Token::Quote,
            ]),
            Program {
                expressions: vec![Expression::String("test".to_string())]
            }
        )
    }
}
