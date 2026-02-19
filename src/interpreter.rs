#[derive(Debug)]
enum Expr {
    Number(i64),
    Binary {
        left: Box<Expr>,
        operation: Operation,
        right: Box<Expr>,
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add
}

#[derive(Debug, Clone)]
enum Token {
    Number(i64),
    Operation(Operation)
}

pub fn execute(code_to_execute: &str) -> i64 {
    let tokens = tokenize(code_to_execute);

    let ast = parse(tokens);

   eval(&ast)
}

fn tokenize(code_to_execute: &str) -> Vec<Token> {
    let words: Vec<&str> = code_to_execute.split(" ").collect();
    words.iter().map(|word| {
        if *word == "+" {
            return Token::Operation(Operation::Add)
        }

        if word.parse::<i64>().is_ok() {
            return Token::Number(word.parse().unwrap())
        }

        panic!("invalid syntax {}", word)
    }).collect()

}

// TODO the parser is super simple and hardcoded. It expects exactly 3 tokens which of course
//  doesn't always have to be the case. Next we should allow for "infinite" additions by making
//  the parser recursive.
fn parse(tokens: Vec<Token>) -> Expr {
    if tokens.len() != 3 {
        panic!("We expect exactly 3 tokens")
    }

    //if tokens.len() == 0 {
    //    return expr
    //}

    //let first_token = tokens[0];
    
    // Check if first token is a number
    // Check if second token is an operation
    // Check if third token is a number again

    let left = match &tokens[0] {
        Token::Number(n) => Expr::Number(*n),
        _ => panic!("First token should be a number")
    };

    let operation = match &tokens[1] {
        Token::Operation(operation) => operation.clone(),
        _ => panic!("Second token should be an operation")
    };

    let right = match &tokens[2] {
        Token::Number(n) => Expr::Number(*n),
        _ => panic!("Third token should be a number")
    };


    Expr::Binary {
        left: Box::new(left),
        operation,
        right: Box::new(right)
    }
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Binary { left, operation: op, right } => {
            let left_evaluated = eval(left);
            let right_evaluated = eval(right);

            match op {
                Operation::Add => left_evaluated + right_evaluated
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
}