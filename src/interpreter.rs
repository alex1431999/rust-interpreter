use crate::enums::Expr;
use crate::enums::Operation;
use crate::{parser, tokenizer};

pub fn execute_interpreter(input: &str) -> i64 {
    let tokens = tokenizer::tokenize(input);

    let ast = parser::parse(&tokens);

    interpret(&ast)
}

fn interpret(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Binary {
            left,
            operation: op,
            right,
        } => {
            let left_evaluated = interpret(left);
            let right_evaluated = interpret(right);

            match op {
                Operation::Add => left_evaluated + right_evaluated,
                Operation::Subtract => left_evaluated - right_evaluated,
                Operation::Multiply => left_evaluated * right_evaluated,
                Operation::Divide => left_evaluated / right_evaluated,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_addition() {
        assert_eq!(execute_interpreter("5 + 5"), 10)
    }

    #[test]
    fn three_numbers_addition() {
        assert_eq!(execute_interpreter("5 + 5 + 5"), 15)
    }

    #[test]
    fn basic_subtraction() {
        assert_eq!(execute_interpreter("5 - 5"), 0)
    }

    #[test]
    fn subtraction_advanced() {
        // This makes sure we aren't just resolving from righ to left but respecting math rules
        assert_eq!(execute_interpreter("5 - 5 - 5"), -5)
    }

    #[test]
    fn multiplication() {
        assert_eq!(execute_interpreter("5 * 5"), 25)
    }

    #[test]
    fn multiplication_advanced() {
        // This test makes sure we are respecting math rules and aren't just evaluating from left
        // to right. In this case the equation should be evaluate as 3 + (5 * 5)
        assert_eq!(execute_interpreter("3 + 5 * 5"), 28)
    }

    #[test]
    fn division() {
        assert_eq!(execute_interpreter("10 / 2"), 5)
    }

    #[test]
    fn division_advanced() {
        assert_eq!(execute_interpreter("3 + 10 / 5"), 5)
    }

    #[test]
    fn equation_advanced() {
        assert_eq!(execute_interpreter("3 + 10 / 5 * 10 - 10 / 2"), 18)
    }

    #[test]
    fn deal_with_white_space() {
        assert_eq!(execute_interpreter("5     +   5"), 10)
    }

    #[test]
    fn deal_with_missing_white_space() {
        assert_eq!(execute_interpreter("5+5"), 10)
    }

    #[test]
    fn deal_with_parentheses() {
        assert_eq!(execute_interpreter("2 * (5 + 5)"), 20)
    }

    #[test]
    fn deal_with_multiple_parentheses() {
        assert_eq!(execute_interpreter("2 * (5 + 5) * (5 + 5)"), 200)
    }

    #[test]
    fn nested_parentheses() {
        assert_eq!(execute_interpreter("2 * (3 + (4 * 5))"), 46);
    }
}
