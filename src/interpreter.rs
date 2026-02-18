#[derive(Debug)]
enum Expr {
    Number(i64),
    Binary {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    }
}

#[derive(Debug)]
enum Op {
    Add
}

pub fn execute(code_to_execute: &str) -> i64 {
    let tokens: Vec<&str> = code_to_execute.split(" ").collect();

    let ast = parse(tokens);

   eval(&ast)
}

// TODO the parser is super simple and hardcoded. It expects exactly 3 tokens which of course
//  doesn't always have to be the case. Next we should allow for "infinite" additions by making
//  the parser recursive.
fn parse(tokens: Vec<&str>) -> Expr {
    if tokens.len() != 3 {
        panic!("Tokens need to be of length 3")
    }

    let first_number = tokens[0].parse().expect("First number must be a number");
    let operation = tokens[1];
    let second_number = tokens[2].parse().expect("Second number must be a number");

    let operation_expression = match operation {
        "+" => Op::Add,
        _ => panic!("Unknown operation {}", operation)
    };

    Expr::Binary {
        left: Box::new(Expr::Number(first_number)),
        op: operation_expression,
        right: Box::new(Expr::Number(second_number))
    }
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Binary { left, op, right } => {
            let left_evaluated = eval(left);
            let right_evaluated = eval(right);

            match op {
                Op::Add => left_evaluated + right_evaluated
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
}