use crate::enums::Expression;
use crate::enums::Operation;
use crate::environment::Environment;
use crate::parser::Program;
use crate::{parser, tokenizer};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Value {
    Number(i64),
    Boolean(bool),
}

pub fn execute_interpreter(input: &str) -> Value {
    let tokens = tokenizer::tokenize(input);

    let ast = parser::parse(&tokens);

    let mut env = Environment {
        values: HashMap::new(),
        parent: None,
    };
    interpret(&ast, &mut env)
}

fn interpret(program: &Program, env: &mut Environment) -> Value {
    let mut result: Value = Value::Number(0);

    for expression in &program.expressions {
        result = interpret_expression(&expression, env)
    }

    result
}

fn interpret_expression(expression: &Expression, env: &mut Environment) -> Value {
    match expression {
        Expression::Number(n) => Value::Number(*n),
        Expression::Boolean(boolean) => Value::Boolean(*boolean),
        Expression::Binary {
            left,
            operation,
            right,
        } => {
            let left_evaluated = interpret_expression(left, env);
            let right_evaluated = interpret_expression(right, env);

            if let Value::Number(left_evaluated_number) = left_evaluated {
                if let Value::Number(right_evaluated_number) = right_evaluated {
                    match operation {
                        Operation::Add => {
                            Value::Number(left_evaluated_number + right_evaluated_number)
                        }
                        Operation::Subtract => {
                            Value::Number(left_evaluated_number - right_evaluated_number)
                        }
                        Operation::Multiply => {
                            Value::Number(left_evaluated_number * right_evaluated_number)
                        }
                        Operation::Divide => {
                            Value::Number(left_evaluated_number / right_evaluated_number)
                        }
                    }
                } else {
                    panic!("Right side of numeric operation is not numeric")
                }
            } else {
                panic!("The transpiler does not currently support non numeric binary expressions")
            }
        }
        Expression::Unary {
            operation,
            expression,
        } => {
            let expression_evaluated = interpret_expression(expression, env);

            if let Value::Number(expression_evaluated_number) = expression_evaluated {
                match operation {
                    Operation::Add => Value::Number(expression_evaluated_number),
                    Operation::Subtract => Value::Number(expression_evaluated_number * -1),
                    _ => panic!("You can only use add an subtract for unary operators"),
                }
            } else {
                panic!("Right side of numeric operation is not numeric")
            }
        }
        Expression::Assign { name, value } => {
            let value_evaluated = interpret_expression(value, env);
            env.set(name.clone(), value_evaluated.clone());
            value_evaluated
        }

        Expression::Variable(name) => env
            .get(name)
            .unwrap_or_else(|| panic!("Undefined variable '{}'", name)),

        Expression::Yell { expression } => {
            let value_evaluated = interpret_expression(expression, env);
            println!("{:?}", value_evaluated);

            // 0 just means the program has run successfully
            Value::Number(0)
        }
        Expression::Block { expressions } => {
            let mut result: Value = Value::Number(0);
            let mut child_env = Environment {
                values: HashMap::new(),
                parent: Some(Box::new(env.clone())),
            };

            for expression in expressions {
                result = interpret_expression(expression, &mut child_env)
            }

            result
        }
        Expression::If { condition, block } => {
            let condition_evaluated = interpret_expression(condition, env);

            if let Value::Boolean(condition_evaluated_number) = condition_evaluated {
                if condition_evaluated_number {
                    interpret_expression(block, env)
                } else {
                    Value::Number(0)
                }
            } else {
                panic!("If statements need to evaluate to a boolean")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_addition() {
        assert_eq!(execute_interpreter("5 + 5"), Value::Number(10))
    }

    #[test]
    fn three_numbers_addition() {
        assert_eq!(execute_interpreter("5 + 5 + 5"), Value::Number(15))
    }

    #[test]
    fn basic_subtraction() {
        assert_eq!(execute_interpreter("5 - 5"), Value::Number(0))
    }

    #[test]
    fn subtraction_advanced() {
        // This makes sure we aren't just resolving from righ to left but respecting math rules
        assert_eq!(execute_interpreter("5 - 5 - 5"), Value::Number(-5))
    }

    #[test]
    fn multiplication() {
        assert_eq!(execute_interpreter("5 * 5"), Value::Number(25))
    }

    #[test]
    fn multiplication_advanced() {
        // This test makes sure we are respecting math rules and aren't just evaluating from left
        // to right. In this case the equation should be evaluate as 3 + (5 * 5)
        assert_eq!(execute_interpreter("3 + 5 * 5"), Value::Number(28))
    }

    #[test]
    fn division() {
        assert_eq!(execute_interpreter("10 / 2"), Value::Number(5))
    }

    #[test]
    fn division_advanced() {
        assert_eq!(execute_interpreter("3 + 10 / 5"), Value::Number(5))
    }

    #[test]
    fn equation_advanced() {
        assert_eq!(
            execute_interpreter("3 + 10 / 5 * 10 - 10 / 2"),
            Value::Number(18)
        )
    }

    #[test]
    fn white_space() {
        assert_eq!(execute_interpreter("5     +   5"), Value::Number(10))
    }

    #[test]
    fn missing_white_space() {
        assert_eq!(execute_interpreter("5+5"), Value::Number(10))
    }

    #[test]
    fn parentheses() {
        assert_eq!(execute_interpreter("2 * (5 + 5)"), Value::Number(20))
    }

    #[test]
    fn multiple_parentheses() {
        assert_eq!(
            execute_interpreter("2 * (5 + 5) * (5 + 5)"),
            Value::Number(200)
        )
    }

    #[test]
    fn nested_parentheses() {
        assert_eq!(execute_interpreter("2 * (3 + (4 * 5))"), Value::Number(46));
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
        assert_eq!(execute_interpreter("-5"), Value::Number(-5));
        assert_eq!(execute_interpreter("--5"), Value::Number(5));
        assert_eq!(execute_interpreter("-(2 + 3)"), Value::Number(-5));
        assert_eq!(execute_interpreter("-2 * 3"), Value::Number(-6));
        assert_eq!(execute_interpreter("2 * -3"), Value::Number(-6));
    }

    #[test]
    #[should_panic]
    fn invalid_unary_expression() {
        execute_interpreter("*5");
    }

    #[test]
    fn assignment() {
        assert_eq!(execute_interpreter("remember x = 5"), Value::Number(5));
        assert_eq!(execute_interpreter("remember x = 5 + 5"), Value::Number(10));
        assert_eq!(
            execute_interpreter("remember x = 5 + 5 + 5"),
            Value::Number(15)
        );
    }

    #[test]
    #[should_panic]
    fn undefined_variable() {
        execute_interpreter("x + 5");
    }

    #[test]
    fn multiple_statements() {
        assert_eq!(
            execute_interpreter("remember x = 5; x + 5"),
            Value::Number(10)
        )
    }

    #[test]
    fn yell() {
        assert_eq!(execute_interpreter("yell(5 + 5)"), Value::Number(0));
        assert_eq!(
            execute_interpreter("yell(5 + 5); 10 + 10"),
            Value::Number(20)
        )
    }
    #[test]
    #[should_panic]
    fn yell_without_parentheses() {
        execute_interpreter("yell 5 + 5");
    }

    #[test]
    fn block_scoping() {
        assert_eq!(
            execute_interpreter("{ remember x = 5; { remember x = 10; x }; x }"),
            Value::Number(5)
        );

        assert_eq!(
            execute_interpreter("{ 5 + 5; { 10 + 10; }; }"),
            Value::Number(20)
        );
    }

    #[test]
    #[should_panic]
    fn variable_does_not_escape_scope() {
        execute_interpreter("{ remember x = 5; }; x");
    }

    #[test]
    fn conditions() {
        assert_eq!(execute_interpreter("if (true) { 5 }"), Value::Number(5));
        assert_eq!(execute_interpreter("if (false) { 5 }"), Value::Number(0));
    }
}
