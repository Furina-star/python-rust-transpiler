//! Integration tests for tokenization and error reporting through the public lexer API.

use python_rust_transpiler::{lexer::tokenize, token::Token};

// Check that the public lexer API produces the expected tokens for a return statement.
#[test]
fn tokenizes_using_public_api() {
    let actual = tokenize("return health + 10").unwrap();

    let expected = vec![
        Token::Return,
        Token::Identifier(String::from("health")),
        Token::Plus,
        Token::Integer(10),
    ];

    assert_eq!(actual, expected);
}

// Check that the public lexer API reports an invalid character with its line and column.
#[test]
fn reports_errors_using_public_api() {
    let error = tokenize("x=@").unwrap_err();

    assert_eq!(error, "Line 1, Column 3: Unexpected character: @");
}
