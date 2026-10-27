//! Abstract syntax tree representations for expressions.

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

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Multiply,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[cfg(test)]
mod tests;
