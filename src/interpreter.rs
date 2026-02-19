#[derive(Debug)]
enum Expr {
    Number(i64),
    Binary {
        left: Box<Expr>,
        operation: Operation,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
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
            Err(_) if *word == "+" => return Token::Operation(Operation::Add),
            Err(_) => panic!("Invalid syntax {}", word),
        })
        .collect()
}

fn parse(tokens: &[Token]) -> Expr {
    if tokens.len() == 0 {
        panic!("You need at least one token to parse")
    }

    if tokens.len() == 1 {
        return match &tokens[0] {
            Token::Number(n) => Expr::Number(*n),
            _ => panic!("The last token can only be a number"),
        };
    }

    let current_token = &tokens[0];
    let next_token = &tokens[1];
    let remaining_tokens = &tokens[2..];

    match current_token {
        Token::Number(n) => match next_token {
            Token::Operation(operation) => Expr::Binary {
                left: Box::new(Expr::Number(*n)),
                operation: operation.clone(),
                right: Box::new(parse(remaining_tokens)),
            },
            _ => panic!("a number can only be followed by an operation"),
        },
        _ => panic!("an expression cant start with an operation"),
    }
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
    fn deal_with_white_space() {
        assert_eq!(execute("5     +   5"), 10)
    }
}
