//! Unit tests for parser token navigation.

use super::*;
use crate::lexer::tokenize_with_diagnostics;
use crate::token::Token;

// Check that peeking returns the first token without consuming it.
#[test]
fn peeks_first_token_without_consuming() {
    let tokens = tokenize_with_diagnostics("42").unwrap();
    let parser = Parser::new(tokens);

    let first = parser.peek().unwrap();
    assert_eq!(first.token, Token::Integer(42));

    let second = parser.peek().unwrap();
    assert_eq!(second.token, Token::Integer(42));

    assert_eq!(parser.current, 0);
}

// Check that peeking returns None when the token vector is empty.
#[test]
fn returns_none_when_no_tokens_exist() {
    let parser = Parser::new(Vec::new());

    assert!(parser.peek().is_none());
}
