#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(i64),
    Variable(String),
    Boolean(bool),
    Block {
        expressions: Vec<Expression>,
    },
    Assign {
        name: String,
        value: Box<Expression>,
    },
    Yell {
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operation: Operation,
        right: Box<Expression>,
    },
    Unary {
        operation: Operation,
        expression: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        success_expression: Box<Expression>,
        failure_expression: Option<Box<Expression>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Operation(Operation),
    ParenthesesOpen,
    ParenthesesClosed,
    Identifier(String),
    Equals,
    Equality,
    Remember,
    Semicolon,
    Yell,
    BlockOpen,
    BlockClosed,
    True,
    False,
    If,
    Else,
}
