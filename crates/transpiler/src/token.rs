//! Token types for literals, keywords, operators, punctuation, and indentation in the source language.

// Store the line and column of a position in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}

// Pair a token with its position in the source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocatedToken {
    pub token: Token,
    pub position: SourcePosition,
}

// Represent the literals, keywords, operators, punctuation, and layout tokens recognized by the lexer.
#[derive(Debug, PartialEq, Clone, Eq)]
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

    // Layout and structural
    Newline,
    Indent,
    Dedent,
}

// Check that a located token preserves its token value and source position.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_token_source_position() {
        let located = LocatedToken {
            token: Token::Integer(100),
            position: SourcePosition { line: 2, column: 5 },
        };

        assert_eq!(located.token, Token::Integer(100));
        assert_eq!(located.position.line, 2);
        assert_eq!(located.position.column, 5);
    }
}
