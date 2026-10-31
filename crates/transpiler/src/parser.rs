//! Recursive-descent parser for the restricted Python-like language.

use crate::token::LocatedToken;

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

    // Return the current token and advance to the next position
    pub fn advance(&mut self) -> Option<&LocatedToken> {
        let token = self.tokens.get(self.current)?;
        self.current += 1;
        Some(token)
    }
}

#[cfg(test)]
mod tests;
