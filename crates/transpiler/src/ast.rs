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
}
