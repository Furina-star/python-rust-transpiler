//! Converts source code into tokens, handling indentation, comments, and line continuations.
//! Reports invalid input with source locations.

use crate::diagnostic::Diagnostic;
use crate::token::{LocatedToken, SourcePosition, Token};

// Check the first identifier character.
fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

// Check subsequent identifier characters.
fn is_identifier_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

// Count leading spaces.
fn count_leading_spaces(line: &str) -> usize {
    let mut spaces: usize = 0;

    for ch in line.chars() {
        if ch == ' ' {
            spaces += 1;
        } else {
            break;
        }
    }

    spaces
}

// Store the column number and message for an error found while tokenizing a line.
#[derive(Debug)]
struct ScanError {
    column: usize,
    message: String,
}

// Convert line-scanning errors into diagnostics with full source positions.
impl ScanError {
    fn into_diagnostic(self, line_number: usize) -> Diagnostic {
        Diagnostic {
            position: SourcePosition {
                line: line_number,
                column: self.column,
            },
            message: self.message,
        }
    }
}

// Convert a single line into tokens with source positions, ignoring spaces and comments.
fn tokenize_line_located(source: &str, line_number: usize) -> Result<Vec<LocatedToken>, ScanError> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().enumerate().peekable();

    // Read characters with lookahead as needed.
    while let Some((index, ch)) = chars.next() {
        let column = index + 1;

        let token = match ch {
            // Read an integer literal and reject invalid suffixes or overflow.
            '0'..='9' => {
                // Collect consecutive ASCII digits.
                let mut digits = String::new();
                digits.push(ch);

                while let Some(&(_, next_ch)) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        digits.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Reject letters or underscores immediately after digits.
                if matches!(chars.peek(), Some(&(_, next_ch)) if is_identifier_start(next_ch)) {
                    return Err(ScanError {
                        column,
                        message: String::from("Invalid integer literal"),
                    });
                }

                // Parse digits and emit an integer token.
                let value = digits.parse::<i32>().map_err(|e| ScanError {
                    column,
                    message: e.to_string(),
                })?;
                Token::Integer(value)
            }
            // Recognize addition.
            '+' => Token::Plus,
            // Distinguish a return type arrow from subtraction.
            '-' => {
                if matches!(chars.peek(), Some(&(_, '>'))) {
                    chars.next();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            // Recognize multiplication.
            '*' => Token::Star,
            // Distinguish floor division from division.
            '/' => {
                if matches!(chars.peek(), Some(&(_, '/'))) {
                    chars.next();
                    Token::FloorDivide
                } else {
                    Token::Slash
                }
            }
            // Recognize modulo.
            '%' => Token::Percent,
            // Distinguish equality from assignment.
            '=' => {
                if matches!(chars.peek(), Some(&(_, '='))) {
                    chars.next();
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            // Recognize less-than comparisons, with optional equality.
            '<' => {
                if matches!(chars.peek(), Some(&(_, '='))) {
                    chars.next();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            // Recognize greater-than comparisons, with optional equality.
            '>' => {
                if matches!(chars.peek(), Some(&(_, '='))) {
                    chars.next();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            // Recognize inequality and reject a standalone exclamation mark.
            '!' => {
                if matches!(chars.peek(), Some(&(_, '='))) {
                    chars.next();
                    Token::NotEqual
                } else {
                    return Err(ScanError {
                        column,
                        message: format!("Unexpected character: {ch}"),
                    });
                }
            }
            // Recognize an opening parenthesis.
            '(' => Token::LeftParen,
            // Recognize a closing parenthesis.
            ')' => Token::RightParen,
            // Recognize a colon.
            ':' => Token::Colon,
            // Recognize a comma.
            ',' => Token::Comma,
            // Reject a carriage return that remains inside a line.
            '\r' => {
                return Err(ScanError {
                    column,
                    message: String::from("Unexpected carriage return"),
                });
            }
            // Read a name and classify it as a keyword, boolean, or identifier.
            c if is_identifier_start(c) => {
                // Collect the full identifier.
                let mut identifier = String::new();
                identifier.push(c);

                while let Some(&(_, next_ch)) = chars.peek() {
                    if is_identifier_continue(next_ch) {
                        identifier.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Classify keywords, booleans, and identifiers.
                match identifier.as_str() {
                    "def" => Token::Def,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "and" => Token::And,
                    "or" => Token::Or,
                    "not" => Token::Not,
                    "True" => Token::Boolean(true),
                    "False" => Token::Boolean(false),
                    _ => Token::Identifier(identifier),
                }
            }
            // Ignore the rest of this physical line after a comment marker.
            '#' => break,
            // Reject tabs in code.
            '\t' => {
                return Err(ScanError {
                    column,
                    message: String::from("Tabs are not supported; use spaces"),
                });
            }
            // Skip spaces between tokens.
            ' ' => continue,
            // Reject unsupported characters.
            _ => {
                return Err(ScanError {
                    column,
                    message: format!("Unexpected character: {ch}"),
                });
            }
        };

        tokens.push(LocatedToken {
            token,
            position: SourcePosition {
                line: line_number,
                column,
            },
        });
    }
    Ok(tokens)
}

// Convert source code into positioned tokens, returning structured diagnostics for invalid input.
pub fn tokenize_with_diagnostics(source: &str) -> Result<Vec<LocatedToken>, Diagnostic> {
    let mut tokens = Vec::new();
    let mut previous_code_line: Option<(usize, usize)> = None;
    let mut levels: Vec<usize> = vec![0];
    let mut opening_lines: Vec<SourcePosition> = Vec::new();

    for (index, line) in source.lines().enumerate() {
        let line_number = index + 1;
        let content = line.trim_start_matches(' ');

        // Ignore blank and comment-only lines without changing indentation.
        if content.is_empty() || content.starts_with('#') {
            continue;
        }

        // Treat lines inside open parentheses as continuations.
        let continuation = !opening_lines.is_empty();
        let line_tokens = tokenize_line_located(line, line_number)
            .map_err(|error| error.into_diagnostic(line_number))?;

        // Separate code lines with a newline token unless the expression continues.
        if !continuation && let Some((previous_line, previous_length)) = previous_code_line {
            tokens.push(LocatedToken {
                token: Token::Newline,
                position: SourcePosition {
                    line: previous_line,
                    column: previous_length + 1,
                },
            });
        }

        let spaces = count_leading_spaces(line);

        // Apply indentation changes only outside parentheses.
        if !continuation {
            if spaces > *levels.last().unwrap() {
                levels.push(spaces);

                tokens.push(LocatedToken {
                    token: Token::Indent,
                    position: SourcePosition {
                        line: line_number,
                        column: spaces + 1,
                    },
                });
            } else if spaces < *levels.last().unwrap() {
                // A dedent must return to an existing indentation level.
                if !levels.contains(&spaces) {
                    let diagnostic = Diagnostic {
                        position: SourcePosition {
                            line: line_number,
                            column: spaces + 1,
                        },
                        message: format!("Inconsistent indentation: {spaces} spaces"),
                    };

                    return Err(diagnostic);
                }

                // Emit one Dedent for each block being closed.
                while *levels.last().unwrap() > spaces {
                    levels.pop().unwrap();

                    tokens.push(LocatedToken {
                        token: Token::Dedent,
                        position: SourcePosition {
                            line: line_number,
                            column: spaces + 1,
                        },
                    });
                }
            }
        }

        // Track open parentheses and reject unmatched closing parentheses.
        for item in &line_tokens {
            match &item.token {
                Token::LeftParen => {
                    opening_lines.push(item.position);
                }

                Token::RightParen if opening_lines.pop().is_none() => {
                    let diagnostic = Diagnostic {
                        position: item.position,
                        message: String::from("Unmatched closing parenthesis"),
                    };

                    return Err(diagnostic);
                }

                _ => {}
            }
        }

        tokens.extend(line_tokens);
        previous_code_line = Some((line_number, line.chars().count()));
    }

    // Report an unclosed parenthesis at its opening position.
    if let Some(&opening_position) = opening_lines.last() {
        let diagnostic = Diagnostic {
            position: opening_position,
            message: String::from("Unclosed parenthesis"),
        };

        return Err(diagnostic);
    }

    // Place final tokens after the last code line, or at the start of empty input.
    let end_position = previous_code_line
        .map(|(line, length)| SourcePosition {
            line,
            column: length + 1,
        })
        .unwrap_or(SourcePosition { line: 1, column: 1 });

    // End the last statement before closing any remaining blocks.
    if previous_code_line.is_some() {
        tokens.push(LocatedToken {
            token: Token::Newline,
            position: end_position,
        });
    }

    // Close all remaining indented blocks.
    while levels.len() > 1 {
        levels.pop();

        tokens.push(LocatedToken {
            token: Token::Dedent,
            position: end_position,
        });
    }

    // Always terminate the token stream with EOF.
    tokens.push(LocatedToken {
        token: Token::Eof,
        position: end_position,
    });

    Ok(tokens)
}

// Return positioned tokens and format diagnostics as error strings.
pub fn tokenize_with_positions(source: &str) -> Result<Vec<LocatedToken>, String> {
    tokenize_with_diagnostics(source).map_err(|diagnostic| diagnostic.to_string())
}

// Return token values without source positions, preserving formatted error messages.
pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    tokenize_with_positions(source)
        .map(|located| located.into_iter().map(|item| item.token).collect())
}

#[cfg(test)]
mod tests;
