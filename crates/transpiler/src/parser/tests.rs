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

// Check that token navigation safely handles an empty vector.
#[test]
fn returns_none_when_no_tokens_exist() {
    let mut parser = Parser::new(Vec::new());

    assert!(parser.peek().is_none());
    assert!(parser.advance().is_none());
    assert_eq!(parser.current, 0);
}

// Check that advancing consumes the current token and moves to the next.
#[test]
fn advances_to_next_token() {
    let tokens = tokenize_with_diagnostics("42").unwrap();
    let mut parser = Parser::new(tokens);

    assert_eq!(parser.advance().unwrap().token, Token::Integer(42));

    assert_eq!(parser.peek().unwrap().token, Token::Newline);
    assert_eq!(parser.current, 1);
}

// Check that advancing past the final EOF token safely returns None.
#[test]
fn advancing_past_eof_returns_none() {
    let tokens = tokenize_with_diagnostics("42").unwrap();
    let mut parser = Parser::new(tokens);

    assert_eq!(parser.advance().unwrap().token, Token::Integer(42));
    assert_eq!(parser.advance().unwrap().token, Token::Newline);
    assert_eq!(parser.advance().unwrap().token, Token::Eof);

    assert!(parser.advance().is_none());
    assert!(parser.peek().is_none());

    assert_eq!(parser.current, 3);
}
