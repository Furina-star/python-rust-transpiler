//! Abstract syntax tree representations for expressions and statements.

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

    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
}

// Represent statements in the syntax tree
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Return { value: Expr },
    Assign { name: String, value: Expr },
}

// Represent operators for binary expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Multiply,
    Subtract,
    Divide,
    FloorDivide,
    Modulo,

    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical
    And,
    Or,
}

// Represent operators for unary expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[cfg(test)]
mod tests;
