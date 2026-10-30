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

// Check that a return statement preserves its expression.
#[test]
fn stores_return_statement() {
    let statement = Stmt::Return {
        value: Expr::Identifier(String::from("total")),
    };

    match statement {
        Stmt::Return { value } => {
            assert_eq!(value, Expr::Identifier(String::from("total")));
        }
        _ => panic!("Expected return statement"),
    }
}

// Check that an assignment statement preserves its variable name and value.
#[test]
fn stores_assignment_statement() {
    let statement = Stmt::Assign {
        name: String::from("total"),
        value: Expr::Integer(42),
    };

    match statement {
        Stmt::Assign { name, value } => {
            assert_eq!(name, String::from("total"));
            assert_eq!(value, Expr::Integer(42));
        }
        _ => panic!("Expected assignment statement"),
    }
}

// Check that a typed variable declaration preserves its name, type, and value.
#[test]
fn stores_typed_variable_declaration() {
    let statement = Stmt::VarDecl {
        name: String::from("total"),
        type_name: String::from("int"),
        value: Expr::Integer(42),
    };

    match statement {
        Stmt::VarDecl {
            name,
            type_name,
            value,
        } => {
            assert_eq!(name, String::from("total"));
            assert_eq!(type_name, String::from("int"));
            assert_eq!(value, Expr::Integer(42));
        }

        _ => panic!("Expected typed variable declaration"),
    }
}

// Check that an if statement preserves its condition and both branches.
#[test]
fn stores_if_else_statement() {
    let condition = Expr::Boolean(true);

    let statement = Stmt::If {
        condition: condition,
        then_branch: vec![Stmt::Return {
            value: Expr::Integer(1),
        }],
        else_branch: vec![Stmt::Return {
            value: Expr::Integer(0),
        }],
    };

    match statement {
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            assert_eq!(condition, Expr::Boolean(true));
            assert_eq!(
                then_branch,
                vec![Stmt::Return {
                    value: Expr::Integer(1)
                }]
            );
            assert_eq!(
                else_branch,
                vec![Stmt::Return {
                    value: Expr::Integer(0)
                }]
            );
        }
        _ => panic!("Expected if statement"),
    }
}

// Check that a while statement preserves its condition and body.
#[test]
fn stores_while_statement() {
    let condition = Expr::Identifier(String::from("active"));

    let body = vec![Stmt::Assign {
        name: String::from("count"),
        value: Expr::Integer(1),
    }];

    let statement = Stmt::While { condition, body };

    match statement {
        Stmt::While { condition, body } => {
            assert_eq!(condition, Expr::Identifier(String::from("active")));
            assert_eq!(
                body,
                vec![Stmt::Assign {
                    name: String::from("count"),
                    value: Expr::Integer(1),
                }]
            );
        }
        _ => panic!("Expected while statement"),
    }
}

// Check that a typed function definition preserves its signature and body.
#[test]
fn stores_typed_function_definition() {
    let parameters = vec![Parameter {
        name: String::from("value"),
        type_name: String::from("int"),
    }];

    let body = vec![Stmt::Return {
        value: Expr::Identifier(String::from("value")),
    }];

    let statement = Stmt::FunctionDef {
        name: String::from("identity"),
        parameters,
        return_type: String::from("int"),
        body,
    };

    match statement {
        Stmt::FunctionDef {
            name,
            parameters,
            return_type,
            body,
        } => {
            assert_eq!(name, "identity");
            assert_eq!(
                parameters,
                vec![Parameter {
                    name: String::from("value"),
                    type_name: String::from("int")
                }]
            );
            assert_eq!(return_type, "int");
            assert_eq!(
                body,
                vec![Stmt::Return {
                    value: Expr::Identifier(String::from("value"))
                }]
            );
        }
        _ => panic!("Expected function definition"),
    }
}
