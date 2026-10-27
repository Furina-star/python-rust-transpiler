//! Abstract syntax tree representations for expressions.

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i32),
    Boolean(bool),

    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
}

#[cfg(test)]
mod tests {
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
}
