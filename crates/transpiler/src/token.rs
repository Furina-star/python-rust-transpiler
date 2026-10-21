#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i32),
    Plus,
    Minus,
    Identifier(String),
    Def,
    Return,
    LeftParen,
    RightParen,
    Colon,
    Comman,
}
