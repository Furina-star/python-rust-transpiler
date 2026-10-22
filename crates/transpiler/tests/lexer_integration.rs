use python_rust_transpiler::{lexer::tokenize, token::Token};

// Temporary
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

#[test]
fn reports_errors_using_public_api() {
    let error = tokenize("x=@").unwrap_err();

    assert_eq!(error, "Line 1, Column 3: Unexpected character: @");
}
