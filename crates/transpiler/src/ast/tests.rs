//! Unit tests for abstract syntax tree structures.

use super::*;

// Check that an integer expression preserves its literal value.
#[test]
fn stores_integer_literal() {
    let expression = Expr::Integer(42);

    assert_eq!(expression, Expr::Integer(42));
}

// Check that a boolean expression preserves its literal value.
#[test]
fn stores_boolean_literal() {
    let expression = Expr::Boolean(true);

    assert_eq!(expression, Expr::Boolean(true));
}

// Check that a binary addition expression preserves its operands and operator.
#[test]
fn stores_binary_addition() {
    let left = Box::new(Expr::Integer(10));
    let right = Box::new(Expr::Integer(5));

    let expression = Expr::Binary {
        left,
        operator: BinaryOp::Add,
        right,
    };

    match expression {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            assert_eq!(*left, Expr::Integer(10));
            assert_eq!(operator, BinaryOp::Add);
            assert_eq!(*right, Expr::Integer(5));
        }
        _ => panic!("Expected a binary expression"),
    }
}

// Check that a binary expression can contain another binary expression.
#[test]
fn stores_nested_binary_addition() {
    let inner = Expr::Binary {
        left: Box::new(Expr::Integer(5)),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Integer(2)),
    };

    let outer = Expr::Binary {
        left: Box::new(Expr::Integer(10)),
        operator: BinaryOp::Add,
        right: Box::new(inner),
    };

    match outer {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            assert_eq!(*left, Expr::Integer(10));
            assert_eq!(operator, BinaryOp::Add);

            // Here, right contains another Expr::Binary.
            match *right {
                Expr::Binary {
                    left: inner_left,
                    operator: inner_operator,
                    right: inner_right,
                } => {
                    assert_eq!(*inner_left, Expr::Integer(5));
                    assert_eq!(inner_operator, BinaryOp::Add);
                    assert_eq!(*inner_right, Expr::Integer(2));
                }

                _ => panic!("Expected inner binary expression"),
            }
        }
        _ => panic!("Expected outer binary expression"),
    }
}

// Check that an addition expression can contain a multiplication expression.
#[test]
fn stores_multiplication_inside_addition() {
    let multiplied = Expr::Binary {
        left: Box::new(Expr::Integer(5)),
        operator: BinaryOp::Multiply,
        right: Box::new(Expr::Integer(2)),
    };

    let expression = Expr::Binary {
        left: Box::new(Expr::Integer(10)),
        operator: BinaryOp::Add,
        right: Box::new(multiplied),
    };

    match expression {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            assert_eq!(*left, Expr::Integer(10));
            assert_eq!(operator, BinaryOp::Add);

            match *right {
                Expr::Binary {
                    left: inner_left,
                    operator: inner_operator,
                    right: inner_right,
                } => {
                    assert_eq!(*inner_left, Expr::Integer(5));
                    assert_eq!(inner_operator, BinaryOp::Multiply);
                    assert_eq!(*inner_right, Expr::Integer(2));
                }

                _ => panic!("Expected multiplication"),
            }
        }
        _ => panic!("Expected addition"),
    }
}

// Check that a unary negation expression preserves its operand and operator.
#[test]
fn stores_unary_negation() {
    let expression = Expr::Unary {
        operator: UnaryOp::Negate,
        operand: Box::new(Expr::Integer(5)),
    };

    match expression {
        Expr::Unary { operator, operand } => {
            assert_eq!(operator, UnaryOp::Negate);
            assert_eq!(*operand, Expr::Integer(5));
        }
        _ => panic!("Expected unary expression"),
    }
}

// Check that logical NOT preserves its operator and boolean operand.
#[test]
fn stores_unary_not() {
    let expression = Expr::Unary {
        operator: UnaryOp::Not,
        operand: Box::new(Expr::Boolean(true)),
    };

    match expression {
        Expr::Unary { operator, operand } => {
            assert_eq!(operator, UnaryOp::Not);
            assert_eq!(*operand, Expr::Boolean(true));
        }
        _ => panic!("Expected unary expression"),
    }
}

// Check that an identifier expression preserves its variable name.
#[test]
fn stores_identifier_expression() {
    let expression = Expr::Identifier(String::from("total"));

    assert_eq!(expression, Expr::Identifier(String::from("total")));
}

// Check that floor division preserves its operands and operator.
#[test]
fn stores_floor_division() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Integer(9)),
        operator: BinaryOp::FloorDivide,
        right: Box::new(Expr::Integer(2)),
    };

    match expression {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            assert_eq!(*left, Expr::Integer(9));
            assert_eq!(operator, BinaryOp::FloorDivide);
            assert_eq!(*right, Expr::Integer(2));
        }
        _ => panic!("Expected binary expression"),
    }
}

// Check that a comparison expression preserves its operands and operator.
#[test]
fn stores_greater_equal_comparison() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Identifier(String::from("age"))),
        operator: BinaryOp::GreaterEqual,
        right: Box::new(Expr::Integer(18)),
    };

    match expression {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            assert_eq!(*left, Expr::Identifier(String::from("age")));
            assert_eq!(operator, BinaryOp::GreaterEqual);
            assert_eq!(*right, Expr::Integer(18))
        }
        _ => panic!("Expected comparison expression"),
    }
}

// Check that a logical AND expression preserves its operands and operator.
#[test]
fn stores_logical_and() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Identifier(String::from("is_student"))),
        operator: BinaryOp::And,
        right: Box::new(Expr::Identifier(String::from("is_active"))),
    };

    let expected = Expr::Binary {
        left: Box::new(Expr::Identifier(String::from("is_student"))),
        operator: BinaryOp::And,
        right: Box::new(Expr::Identifier(String::from("is_active"))),
    };

    assert_eq!(expression, expected);
}

// Check that a function call preserves its callee and arguments.
#[test]
fn stores_function_call() {
    let expression = Expr::Call {
        callee: Box::new(Expr::Identifier(String::from("add"))),
        arguments: vec![Expr::Integer(10), Expr::Integer(5)],
    };

    match expression {
        Expr::Call { callee, arguments } => {
            assert_eq!(*callee, Expr::Identifier(String::from("add")));
            assert_eq!(arguments, vec![Expr::Integer(10), Expr::Integer(5)]);
        }
        _ => panic!("Expected function call"),
    }
}
