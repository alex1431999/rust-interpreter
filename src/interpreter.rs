use crate::enums::Expr;
use crate::enums::Operation;
use crate::enums::Token;
use crate::parser::parse;

pub fn execute(code_to_execute: &str) -> i64 {
    let tokens = tokenize(code_to_execute);

    let ast = parse(&tokens);

    eval(&ast)
}

fn tokenize(code_to_execute: &str) -> Vec<Token> {
    let words: Vec<&str> = code_to_execute.split_whitespace().collect();
    words
        .iter()
        .map(|word| match word.parse::<i64>() {
            Ok(n) => Token::Number(n),
            Err(_) if *word == "+" => Token::Operation(Operation::Add),
            Err(_) if *word == "-" => Token::Operation(Operation::Subtract),
            Err(_) if *word == "*" => Token::Operation(Operation::Multiply),
            Err(_) => panic!("Invalid syntax {}", word),
        })
        .collect()
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Binary {
            left,
            operation: op,
            right,
        } => {
            let left_evaluated = eval(left);
            let right_evaluated = eval(right);

            match op {
                Operation::Add => left_evaluated + right_evaluated,
                Operation::Subtract => left_evaluated - right_evaluated,
                Operation::Multiply => left_evaluated * right_evaluated,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_addition() {
        assert_eq!(execute("5 + 5"), 10)
    }

    #[test]
    fn three_numbers_addition() {
        assert_eq!(execute("5 + 5 + 5"), 15)
    }

    #[test]
    fn basic_subtraction() {
        assert_eq!(execute("5 - 5"), 0)
    }

    #[test]
    fn subtraction_advanced() {
        // This makes sure we aren't just resolving from righ to left but respecting math rules
        assert_eq!(execute("5 - 5 - 5"), -5)
    }

    #[test]
    fn multiplication() {
        assert_eq!(execute("5 * 5"), 25)
    }

    #[test]
    fn multiplication_advanced() {
        // This test makes sure we are respecting math rules and aren't just evaluating from left
        // to right. In this case the equation should be evaluate as 3 + (5 * 5)
        assert_eq!(execute("3 + 5 * 5"), 28)
    }

    #[test]
    fn deal_with_white_space() {
        assert_eq!(execute("5     +   5"), 10)
    }
}
