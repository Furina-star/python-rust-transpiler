//! Abstract syntax tree expressions for integer and boolean literals.

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i32),
    Boolean(bool),
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
}
