use crate::enums::{Comparator, Expression};
use crate::enums::{Operation, Value};
use crate::environment::{Environment, EnvironmentRecord};
use crate::parser::Program;
use crate::{parser, tokenizer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn execute_interpreter(input: &str) -> Value {
    let tokens = tokenizer::tokenize(input);

    let ast = parser::parse(&tokens);

    let env = Rc::new(RefCell::new(Environment {
        records: HashMap::new(),
        parent: None,
    }));

    interpret(&ast, &env)
}

fn interpret(program: &Program, env: &Rc<RefCell<Environment>>) -> Value {
    let mut result: Value = Value::Number(0);

    for expression in &program.expressions {
        result = interpret_expression(&expression, env)
    }

    result
}

fn interpret_expression(expression: &Expression, env: &Rc<RefCell<Environment>>) -> Value {
    match expression {
        Expression::Number(n) => Value::Number(*n),
        Expression::Float(f) => Value::Float(*f),
        Expression::Boolean(boolean) => Value::Boolean(*boolean),
        Expression::String(string) => Value::String(string.clone()),
        Expression::List(items) => {
            let mut list: Vec<Value> = vec![];
            for item in items {
                let item_evaluated = interpret_expression(item, env);
                list.push(item_evaluated);
            }

            Value::List(list)
        }
        Expression::Null => Value::Null,
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
            env.borrow_mut().set(
                name.clone(),
                EnvironmentRecord::Value(value_evaluated.clone()),
            );
            value_evaluated
        }

        Expression::Variable(name) => {
            match env
                .borrow()
                .get(name)
                .unwrap_or_else(|| panic!("Undefined variable '{}'", name))
            {
                EnvironmentRecord::Value(value) => value,
                _ => panic!("Undefined variable '{}'", name),
            }
        }

        Expression::Yell { expression } => {
            let value_evaluated = interpret_expression(expression, env);
            println!("{:?}", value_evaluated);

            Value::Null
        }
        Expression::Block { expressions } => {
            let mut result: Value = Value::Null;
            let child_env = Rc::new(RefCell::new(Environment {
                records: HashMap::new(),
                parent: Some(env.clone()),
            }));

            for expression in expressions {
                result = interpret_expression(expression, &child_env)
            }

            result
        }
        Expression::If {
            condition,
            success_expression,
            failure_expression,
        } => {
            let condition_evaluated = interpret_expression(condition, env);

            if let Value::Boolean(condition_evaluated_resolved) = condition_evaluated {
                if condition_evaluated_resolved {
                    interpret_expression(success_expression, env)
                } else if let Some(failure_expression_resolved) = failure_expression {
                    interpret_expression(failure_expression_resolved, env)
                } else {
                    Value::Null
                }
            } else {
                panic!("If statements need to evaluate to a boolean")
            }
        }
        Expression::Comparison {
            left,
            comparator,
            right,
        } => {
            let left_evaluated = interpret_expression(left, env);
            let right_evaluated = interpret_expression(right, env);

            match comparator {
                Comparator::Equality => Value::Boolean(left_evaluated == right_evaluated),
                Comparator::GreaterThan => match (left_evaluated, right_evaluated) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left > right),
                    (Value::Number(_), _) => {
                        panic!("Right side of greater than comparison needs to be a number")
                    }
                    (_, _) => {
                        panic!("Left side of greater than comparison needs to be a number")
                    }
                },
                Comparator::LessThan => match (left_evaluated, right_evaluated) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left < right),
                    (Value::Number(_), _) => {
                        panic!("Right side of less than comparison needs to be a number")
                    }
                    (_, _) => {
                        panic!("Left side of less than comparison needs to be a number")
                    }
                },
            }
        }
        Expression::While {
            condition,
            expression,
        } => {
            let mut condition_evaluated = interpret_expression(condition, env);
            let mut continue_loop = is_truthy_value(condition_evaluated);

            while continue_loop {
                interpret_expression(expression, env);

                condition_evaluated = interpret_expression(condition, env);
                continue_loop = is_truthy_value(condition_evaluated);
            }

            Value::Null
        }
        Expression::For {
            identifier,
            list,
            expression,
        } => {
            match interpret_expression(list, env) {
                Value::List(list_evaluated) => {
                    for item in list_evaluated {
                        env.borrow_mut()
                            .set(identifier.clone(), EnvironmentRecord::Value(item));
                        interpret_expression(expression, env);
                    }
                }
                _ => panic!("for loop needs to iterate over a list"),
            }

            Value::Null
        }
        Expression::Function {
            identifier,
            parameters,
            expression,
        } => {
            env.borrow_mut().set(
                identifier.clone(),
                EnvironmentRecord::Function {
                    parameters: parameters.clone(),
                    expression: expression.clone(),
                },
            );

            Value::Null
        }
        Expression::FunctionCall {
            identifier,
            parameters,
        } => {
            match env
                .borrow()
                .get(identifier)
                .unwrap_or_else(|| panic!("Undefined variable '{}'", identifier))
            {
                EnvironmentRecord::Function {
                    parameters: parameter_names,
                    expression,
                } => {
                    if parameters.len() != parameter_names.len() {
                        panic!(
                            "Incorrect amount of parameters supplied for function {}",
                            identifier
                        );
                    }

                    let child_env = Rc::new(RefCell::new(Environment {
                        records: HashMap::new(),
                        parent: Some(env.clone()),
                    }));

                    let mut i = 0;
                    while i < parameters.len() {
                        let parameter = &parameters[i];
                        let parameter_name = &parameter_names[i];
                        let parameter_resolved = interpret_expression(parameter, env);

                        child_env.borrow_mut().records.insert(
                            parameter_name.clone(),
                            EnvironmentRecord::Value(parameter_resolved),
                        );

                        i += 1;
                    }

                    interpret_expression(&*expression, &child_env)
                }
                _ => panic!("Undefined variable '{}'", identifier),
            }
        }
    }
}

fn is_truthy_value(value: Value) -> bool {
    match value {
        Value::Number(number) => number > 0,
        Value::Float(float) => float > 0.0,
        Value::Boolean(bool) => bool,
        Value::String(string) => string != "",
        Value::Null => false,
        Value::List(list) => list.len() > 0,
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
    fn re_assignment() {
        assert_eq!(
            execute_interpreter("remember x = 5; x = 10; x"),
            Value::Number(10)
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
        assert_eq!(execute_interpreter("yell(5 + 5)"), Value::Null);
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
        assert_eq!(execute_interpreter("if (false) { 5 }"), Value::Null);
        assert_eq!(
            execute_interpreter("if (false) { 5 } else { 10 }"),
            Value::Number(10)
        );
        assert_eq!(execute_interpreter("if (5 == 5) { 5 }"), Value::Number(5));
        assert_eq!(
            execute_interpreter("remember x = 5; if (x + 5 == 10) { 5 }"),
            Value::Number(5)
        );
    }

    #[test]
    fn equality() {
        assert_eq!(execute_interpreter("5 == 5"), Value::Boolean(true));
        assert_eq!(execute_interpreter("5 == 4"), Value::Boolean(false));
        assert_eq!(execute_interpreter("true == true"), Value::Boolean(true));
        assert_eq!(execute_interpreter("true == false"), Value::Boolean(false));
        assert_eq!(execute_interpreter("false == false"), Value::Boolean(true));
        assert_eq!(execute_interpreter("5 == true"), Value::Boolean(false));
    }

    #[test]
    fn comparators() {
        assert_eq!(execute_interpreter("5 > 3"), Value::Boolean(true));
        assert_eq!(execute_interpreter("5 < 3"), Value::Boolean(false));
        assert_eq!(execute_interpreter("0 > 3"), Value::Boolean(false));
        assert_eq!(execute_interpreter("0 < 3"), Value::Boolean(true));
    }

    #[test]
    #[should_panic]
    fn comparators_invalid() {
        execute_interpreter("5 > true");
    }

    #[test]
    fn strings() {
        assert_eq!(
            execute_interpreter("\"hello\""),
            Value::String("hello".to_string())
        );
        assert_eq!(
            execute_interpreter("remember x = \"hello\"; x;"),
            Value::String("hello".to_string())
        )
    }

    #[test]
    #[should_panic]
    fn invalid_string() {
        execute_interpreter("\"test");
    }

    #[test]
    fn while_loop() {
        assert_eq!(
            execute_interpreter("remember x = 0; while (x < 5) { x = x + 1 }; x;"),
            Value::Number(5)
        )
    }

    #[test]
    fn null() {
        assert_eq!(execute_interpreter("null"), Value::Null)
    }

    #[test]
    fn lists() {
        assert_eq!(
            execute_interpreter("[1, 2, 3]"),
            Value::List(vec![Value::Number(1), Value::Number(2), Value::Number(3)])
        );
        assert_eq!(
            execute_interpreter("remember x = [1, 2, 3]; x;"),
            Value::List(vec![Value::Number(1), Value::Number(2), Value::Number(3)])
        )
    }

    #[test]
    #[should_panic]
    fn invalid_list_missing_closing_bracket() {
        execute_interpreter("[1,2");
    }

    #[test]
    #[should_panic]
    fn invalid_list_missing_comma() {
        execute_interpreter("[1 2]");
    }

    #[test]
    fn for_loop() {
        assert_eq!(
            execute_interpreter("for (x in [1]) { yell(x); };"),
            Value::Null
        );

        assert_eq!(
            execute_interpreter("remember x = 0; for (y in [5, 6]) { x = y }; y;"),
            Value::Number(6)
        )
    }

    #[test]
    fn float() {
        assert_eq!(execute_interpreter("5.5"), Value::Float(5.5))
    }

    #[test]
    fn functions() {
        assert_eq!(
            execute_interpreter("function test() { 5 }; test()"),
            Value::Number(5)
        );
    }

    #[test]
    fn functions_with_params() {
        assert_eq!(
            execute_interpreter("function test(x) { x + 5 }; test(2)"),
            Value::Number(7)
        );
        assert_eq!(
            execute_interpreter("function add(a, b) { a + b }; add(5, 5)"),
            Value::Number(10)
        )
    }

    #[test]
    fn functions_variable_scope() {
        assert_eq!(
            execute_interpreter("remember x = 5; function test(x) { x  + 5 }; test(2)"),
            Value::Number(7)
        );
        assert_eq!(
            execute_interpreter("remember y = 5; function test(x) { x  + y }; test(2)"),
            Value::Number(7)
        )
    }
}
