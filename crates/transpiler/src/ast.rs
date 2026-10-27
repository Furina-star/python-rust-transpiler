//! Abstract syntax tree representations for expressions.

// Represent expressions in the syntax tree.
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

// Represent operators for binary expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Multiply,
    Subtract,
    Divide,
    FloorDivide,
    Modulo,
}

// Represent operators for unary expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[cfg(test)]
mod tests;
