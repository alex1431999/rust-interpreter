use crate::enums::Expression;
use crate::enums::Operation;
use crate::parser::Program;
use crate::{parser, tokenizer};
use std::collections::HashMap;

struct Environment {
    values: HashMap<String, i64>,
}

pub fn execute_interpreter(input: &str) -> i64 {
    let tokens = tokenizer::tokenize(input);

    let ast = parser::parse(&tokens);

    let mut env = Environment {
        values: HashMap::new(),
    };
    interpret(&ast, &mut env)
}

fn interpret(program: &Program, env: &mut Environment) -> i64 {
    let mut result: i64 = 0;

    for expression in &program.expressions {
        result = interpret_expression(&expression, env)
    }

    result
}

fn interpret_expression(expression: &Expression, env: &mut Environment) -> i64 {
    match expression {
        Expression::Number(n) => *n,
        Expression::Binary {
            left,
            operation,
            right,
        } => {
            let left_evaluated = interpret_expression(left, env);
            let right_evaluated = interpret_expression(right, env);

            match operation {
                Operation::Add => left_evaluated + right_evaluated,
                Operation::Subtract => left_evaluated - right_evaluated,
                Operation::Multiply => left_evaluated * right_evaluated,
                Operation::Divide => left_evaluated / right_evaluated,
            }
        }
        Expression::Unary {
            operation,
            expression,
        } => {
            let expression_evaluated = interpret_expression(expression, env);

            match operation {
                Operation::Add => expression_evaluated,
                Operation::Subtract => expression_evaluated * -1,
                _ => panic!("You can only use add an subtract for unary operators"),
            }
        }
        Expression::Assign { name, value } => {
            let value_evaluated = interpret_expression(value, env);
            env.values.insert(name.clone(), value_evaluated);
            value_evaluated
        }

        Expression::Variable(name) => *env
            .values
            .get(name)
            .unwrap_or_else(|| panic!("Undefined variable '{}'", name)),
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
    fn white_space() {
        assert_eq!(execute_interpreter("5     +   5"), 10)
    }

    #[test]
    fn missing_white_space() {
        assert_eq!(execute_interpreter("5+5"), 10)
    }

    #[test]
    fn parentheses() {
        assert_eq!(execute_interpreter("2 * (5 + 5)"), 20)
    }

    #[test]
    fn multiple_parentheses() {
        assert_eq!(execute_interpreter("2 * (5 + 5) * (5 + 5)"), 200)
    }

    #[test]
    fn nested_parentheses() {
        assert_eq!(execute_interpreter("2 * (3 + (4 * 5))"), 46);
    }

    #[test]
    #[should_panic]
    fn missing_open_parentheses() {
        execute_interpreter("2 + 2 + 5)");
    }

    #[test]
    #[should_panic]
    fn missing_closing_parentheses() {
        execute_interpreter("2 + (2 + 5");
    }

    #[test]
    fn unary_expressions() {
        assert_eq!(execute_interpreter("-5"), -5);
        assert_eq!(execute_interpreter("--5"), 5);
        assert_eq!(execute_interpreter("-(2 + 3)"), -5);
        assert_eq!(execute_interpreter("-2 * 3"), -6);
        assert_eq!(execute_interpreter("2 * -3"), -6);
    }

    #[test]
    #[should_panic]
    fn invalid_unary_expression() {
        execute_interpreter("*5");
    }

    #[test]
    fn assignment() {
        assert_eq!(execute_interpreter("remember x = 5"), 5);
        assert_eq!(execute_interpreter("remember x = 5 + 5"), 10);
        assert_eq!(execute_interpreter("remember x = 5 + 5 + 5"), 15);
    }

    #[test]
    #[should_panic]
    fn undefined_variable() {
        execute_interpreter("x + 5");
    }

    #[test]
    fn multiple_statements() {
        assert_eq!(execute_interpreter("remember x = 5; x + 5"), 10)
    }
}
