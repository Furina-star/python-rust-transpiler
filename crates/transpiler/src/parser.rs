//! Recursive-descent parser for the restricted Python-like language.

use crate::token::{LocatedToken, Token};

// Store positioned tokens and track the current parsing position.
pub struct Parser {
    tokens: Vec<LocatedToken>,
    current: usize,
}

impl Parser {
    // Create a parser positioned at the first token.
    pub fn new(tokens: Vec<LocatedToken>) -> Self {
        Self { tokens, current: 0 }
    }

    // Inspect the current token without consuming it.
    pub fn peek(&self) -> Option<&LocatedToken> {
        self.tokens.get(self.current)
    }

    // Return the current token and advance to the next position.
    pub fn advance(&mut self) -> Option<&LocatedToken> {
        let token = self.tokens.get(self.current)?;
        self.current += 1;
        Some(token)
    }

    // Check whether the current token matches the expected results.
    pub fn check(&self, expected: &Token) -> bool {
        self.peek()
            .is_some_and(|located| &located.token == expected)
    }

    // Consume the current token only when it matches the expected token
    pub fn consume_if(&mut self, expected: &Token) -> bool {
        if !self.check(expected) {
            return false;
        }

        self.advance().is_some()
    }
}

#[cfg(test)]
mod tests;
