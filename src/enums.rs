#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Binary {
        left: Box<Expr>,
        operation: Operation,
        right: Box<Expr>,
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
}
