#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(i64),
    Float(f64),
    String(String),
    Variable(String),
    Boolean(bool),
    List(Vec<Expression>),
    Null,
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
    Comparison {
        left: Box<Expression>,
        comparator: Comparator,
        right: Box<Expression>,
    },
    While {
        condition: Box<Expression>,
        expression: Box<Expression>,
    },
    For {
        identifier: String,
        list: Box<Expression>,
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comparator {
    Equality,
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Float(f64),
    Operation(Operation),
    ParenthesesOpen,
    ParenthesesClosed,
    BracketOpen,
    BracketClosed,
    Identifier(String),
    Equals,
    Remember,
    Semicolon,
    Yell,
    BlockOpen,
    BlockClosed,
    True,
    False,
    If,
    Else,
    Comparator(Comparator),
    Quote,
    String(String),
    While,
    Null,
    Comma,
    For,
    In,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    List(Vec<Value>),
    Null,
}
