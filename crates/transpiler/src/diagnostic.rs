//! Diagnostics with source positions and formatted error messages.

use crate::token::SourcePosition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub position: SourcePosition,
    pub message: String,
}

use std::fmt;

impl fmt::Display for Diagnostic {
    // Format the diagnostic as a line, column, and error message.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Line {}, Column {}: {}",
            self.position.line, self.position.column, self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Check that a diagnostic displays its line, column, and message.
    #[test]
    fn formats_diagnostic_with_source_position() {
        let diagnostic = Diagnostic {
            position: SourcePosition { line: 2, column: 5 },
            message: String::from("Unexpected character: @"),
        };

        assert_eq!(
            diagnostic.to_string(),
            "Line 2, Column 5: Unexpected character: @"
        );
    }
}
