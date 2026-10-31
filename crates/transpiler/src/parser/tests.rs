//! Unit tests for parser token navigation.

use super::*;
use crate::lexer::tokenize_with_diagnostics;
use crate::token::{SourcePosition, Token};

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

// Check that matching a token does not advance the parser.
#[test]
fn checks_token_without_consuming() {
    let tokens = tokenize_with_diagnostics("return 42").unwrap();
    let parser = Parser::new(tokens);

    assert!(parser.check(&Token::Return));
    assert!(!parser.check(&Token::Integer(42)));

    assert_eq!(parser.current, 0);
}

// Check that conditional consumption advances only for matching tokens.
#[test]
fn consumes_only_matching_tokens() {
    let tokens = tokenize_with_diagnostics("return 42").unwrap();
    let mut parser = Parser::new(tokens);

    assert!(!parser.consume_if(&Token::Integer(42)));
    assert_eq!(parser.current, 0);

    assert!(parser.consume_if(&Token::Return));
    assert_eq!(parser.current, 1);

    assert!(parser.consume_if(&Token::Integer(42)));
    assert_eq!(parser.current, 2);

    assert_eq!(parser.peek().unwrap().token, Token::Newline);
}

// Check that expecting a matching token consumes it successfully.
#[test]
fn expects_matching_token() {
    let tokens = tokenize_with_diagnostics("return 42").unwrap();
    let mut parser = Parser::new(tokens);

    let token = parser.expect(&Token::Return).unwrap();

    assert_eq!(token.token, Token::Return);
    assert_eq!(parser.current, 1);
    assert_eq!(parser.peek().unwrap().token, Token::Integer(42));
}

// Check that expecting the wrong token reports an error without advancing.
#[test]
fn reports_error_for_unexpected_token() {
    let tokens = tokenize_with_diagnostics("return 42").unwrap();
    let mut parser = Parser::new(tokens);

    let error = parser.expect(&Token::Newline).unwrap_err();

    assert_eq!(error.position, SourcePosition { line: 1, column: 1 });
    assert_eq!(error.message, "Expected Newline, found Return");
    assert_eq!(parser.current, 0);
}

// Check that expecting a token after EOF returns a diagnostic.
#[test]
fn reports_error_after_end_of_stream() {
    let tokens = tokenize_with_diagnostics("42").unwrap();
    let mut parser = Parser::new(tokens);

    parser.advance().unwrap();
    parser.advance().unwrap();
    parser.advance().unwrap();

    let error = parser.expect(&Token::Return).unwrap_err();

    assert_eq!(error.position, SourcePosition { line: 1, column: 3 });
    assert_eq!(error.message, "Expected Return, found end of token stream");
    assert_eq!(parser.current, 3);
}
