// TODO we can pick up from step 3 here -> Extend your AST

#[derive(Debug)]
pub enum Expression {
    Number(i64),
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
}
