#[derive(Debug, PartialEq)]
pub enum Token {
    // Literals and identifiers
    Integer(i32),
    Identifier(String),

    // Keywords
    Def,
    Return,
    If,
    Else,
    While,

    // Delimiters and punctuation
    LeftParen,
    RightParen,
    Colon,
    Comma,
    Arrow,

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    FloorDivide,
    Percent,

    // Assignment operator
    Equal,

    // Comparison operators
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical operators
    And,
    Or,
    Not,

    // Boolean Literals
    Boolean(bool),

    // Escape Sequences
    Newline,
}
