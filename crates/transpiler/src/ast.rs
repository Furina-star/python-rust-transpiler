//! Abstract syntax tree representations for expressions.

// Represent literals, identifiers, binary operations, and unary operations in the syntax tree.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i32),
    Boolean(bool),
    Identifier(String),

    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },

    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },
}

// Represent addition and multiplication operators for binary expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Multiply,
}

// Represent arithmetic negation and logical NOT operators for unary expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[cfg(test)]
mod tests;
