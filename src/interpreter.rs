#[derive(Debug)]
enum Expr {
    Number(i64),
    Binary {
        left: Box<Expr>,
        operation: Operation,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
}

#[derive(Debug, Clone)]
enum Token {
    Number(i64),
    Operation(Operation),
}

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

fn parse(tokens: &[Token]) -> Expr {
    if tokens.is_empty() {
        panic!("You need at least one token to parse")
    }

    let mut index = 0;

    let mut left = match &tokens[index] {
        Token::Number(n) => Expr::Number(*n),
        _ => panic!("The first token must be a number"),
    };

    while index < tokens.len() - 1 {
        index += 1;

        let operation = match &tokens[index] {
            Token::Operation(operation) => *operation,
            _ => panic!("The second token must be an operation"),
        };

        index += 1;

        let right = match &tokens[index] {
            Token::Number(n) => Expr::Number(*n),
            _ => panic!("The third token must be a number"),
        };

        left = Expr::Binary {
            left: Box::new(left),
            operation,
            right: Box::new(right),
        }
    }

    left
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
    fn deal_with_white_space() {
        assert_eq!(execute("5     +   5"), 10)
    }
}
