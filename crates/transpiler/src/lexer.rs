//! Converts source code into tokens, handling indentation, comments, and line continuations.
//! Reports invalid input with source locations.

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

// Push a new indentation level and emit Indent.
fn record_indent_increase(spaces: usize, levels: &mut Vec<usize>, tokens: &mut Vec<Token>) {
    let current = *levels.last().unwrap();

    if spaces > current {
        levels.push(spaces);
        tokens.push(Token::Indent);
    }
}

// Process indentation changes.
fn process_indentation(
    spaces: usize,
    levels: &mut Vec<usize>,
    tokens: &mut Vec<Token>,
) -> Result<(), String> {
    let current = *levels.last().unwrap();

    if spaces > current {
        record_indent_increase(spaces, levels, tokens);
    } else if spaces < current {
        record_indent_decrease(spaces, levels, tokens)?;
    }

    Ok(())
}

// Validate dedentation, pop levels, and emit Dedent tokens.
fn record_indent_decrease(
    spaces: usize,
    levels: &mut Vec<usize>,
    tokens: &mut Vec<Token>,
) -> Result<(), String> {
    let current = *levels.last().unwrap();

    if spaces < current && !levels.contains(&spaces) {
        return Err(format!("Inconsistent indentation: {spaces} spaces"));
    }

    while *levels.last().unwrap() > spaces {
        levels.pop().unwrap();
        tokens.push(Token::Dedent);
    }

    Ok(())
}

// Store the column number and message for an error found while tokenizing a line.
#[derive(Debug)]
struct ScanError {
    column: usize,
    message: String,
}

// Convert a single line into tokens with source positions, ignoring spaces and comments.
fn tokenize_line_located(source: &str, line_number: usize) -> Result<Vec<LocatedToken>, ScanError> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().enumerate().peekable();

    // Read characters with lookahead as needed.
    while let Some((index, ch)) = chars.next() {
        let column = index + 1;

        let token = match ch {
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

                // Check for invalid integer literals
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
            '+' => Token::Plus,
            '-' => {
                if matches!(chars.peek(), Some(&(_, '>'))) {
                    chars.next();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            '*' => Token::Star,
            '/' => {
                if matches!(chars.peek(), Some(&(_, '/'))) {
                    chars.next();
                    Token::FloorDivide
                } else {
                    Token::Slash
                }
            }
            '%' => Token::Percent,
            '=' => {
                if matches!(chars.peek(), Some(&(_, '='))) {
                    chars.next();
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '<' => {
                if matches!(chars.peek(), Some(&(_, '='))) {
                    chars.next();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if matches!(chars.peek(), Some(&(_, '='))) {
                    chars.next();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
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
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '\r' => {
                return Err(ScanError {
                    column,
                    message: String::from("Unexpected carriage return"),
                });
            }
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
            '\t' => {
                return Err(ScanError {
                    column,
                    message: String::from("Tabs are not supported; use spaces"),
                });
            }
            ' ' => continue,
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

// Convert a single line into tokens and discard their source positions.
fn tokenize_line(source: &str) -> Result<Vec<Token>, ScanError> {
    let located = tokenize_line_located(source, 1)?;

    Ok(located.into_iter().map(|item| item.token).collect())
}

// Convert source code line by line into tokens with line numbers and starting columns.
pub fn tokenize_with_positions(source: &str) -> Result<Vec<LocatedToken>, String> {
    let mut tokens = Vec::new();
    let mut previous_code_line: Option<(usize, usize)> = None;
    let mut levels: Vec<usize> = vec![0];

    for (index, line) in source.lines().enumerate() {
        let line_number = index + 1;
        let content = line.trim_start_matches(' ');

        if content.is_empty() || content.starts_with('#') {
            continue;
        }

        let line_tokens = tokenize_line_located(line, line_number).map_err(|error| {
            format!(
                "Line {line_number}, Column {}: {}",
                error.column, error.message
            )
        })?;

        if let Some((previous_line, previous_length)) = previous_code_line {
            tokens.push(LocatedToken {
                token: Token::Newline,
                position: SourcePosition {
                    line: previous_line,
                    column: previous_length + 1,
                },
            });
        }

        let spaces = count_leading_spaces(line);

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
            if !levels.contains(&spaces) {
                return Err(format!(
                    "Line {line_number}: Inconsistent indentation: {spaces} spaces"
                ));
            }

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

        tokens.extend(line_tokens);
        previous_code_line = Some((line_number, line.chars().count()));
    }

    Ok(tokens)
}

// Convert source code into tokens, handling newlines, indentation, and line continuations.
pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut levels: Vec<usize> = vec![0];
    let mut saw_code_line = false;
    let mut opening_lines: Vec<usize> = Vec::new();

    for (index, line) in source.lines().enumerate() {
        let line_number = index + 1;
        let content = line.trim_start_matches(' ');

        if content.is_empty() || content.starts_with('#') {
            continue;
        }

        let continuation = !opening_lines.is_empty();

        if saw_code_line && !continuation {
            tokens.push(Token::Newline);
        }

        saw_code_line = true;

        if !continuation {
            let spaces = count_leading_spaces(line);

            process_indentation(spaces, &mut levels, &mut tokens)
                .map_err(|message| format!("Line {line_number}: {message}"))?;
        }

        let line_tokens = tokenize_line(line).map_err(|error| {
            format!(
                "Line {line_number}, Column {}: {}",
                error.column, error.message
            )
        })?;

        for token in &line_tokens {
            match token {
                Token::LeftParen => {
                    opening_lines.push(line_number);
                }

                Token::RightParen if opening_lines.pop().is_none() => {
                    return Err(format!("Line {line_number}: Unmatched closing parenthesis"));
                }

                _ => {}
            }
        }

        tokens.extend(line_tokens);
    }

    if let Some(&opening_line) = opening_lines.last() {
        return Err(format!("Line {opening_line}: Unclosed parenthesis"));
    }

    record_indent_decrease(0, &mut levels, &mut tokens)?;

    Ok(tokens)
}

#[cfg(test)]
mod tests;
