#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(i64),
    Variable(String),
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
    Remember,
    Semicolon,
    Yell,
    BlockOpen,
    BlockClosed,
    True,
    False,
}
