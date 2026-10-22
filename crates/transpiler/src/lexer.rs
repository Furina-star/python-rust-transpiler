use crate::token::Token;

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

// Convert a single line into tokens, ignoring spaces and comments.
fn tokenize_line(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    // Read characters with lookahead as needed.
    while let Some(ch) = chars.next() {
        match ch {
            '0'..='9' => {
                // Collect consecutive ASCII digits.
                let mut digits = String::new();
                digits.push(ch);

                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        digits.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Parse digits and emit an integer token.
                let value = digits.parse::<i32>().map_err(|e| e.to_string())?;
                let token = Token::Integer(value);
                tokens.push(token);
            }
            '+' => tokens.push(Token::Plus),
            '-' => {
                // Distinguish arrow from minus.
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Minus);
                }
            }
            '*' => tokens.push(Token::Star),
            '/' => {
                // Distinguish floor division from division.
                if chars.peek() == Some(&'/') {
                    chars.next();
                    tokens.push(Token::FloorDivide);
                } else {
                    tokens.push(Token::Slash);
                }
            }
            '%' => tokens.push(Token::Percent),
            '=' => {
                // Distinguish equality from assignment.
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::EqualEqual);
                } else {
                    tokens.push(Token::Equal);
                }
            }
            '<' => {
                // Distinguish <= from <.
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::LessEqual);
                } else {
                    tokens.push(Token::Less);
                }
            }
            '>' => {
                // Distinguish >= from >.
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::GreaterEqual);
                } else {
                    tokens.push(Token::Greater);
                }
            }
            '!' => {
                // Accept != and reject a standalone !.
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::NotEqual);
                } else {
                    return Err(format!("Unexpected character: {ch}"));
                }
            }
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '\n' => tokens.push(Token::Newline),
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                    tokens.push(Token::Newline);
                } else {
                    return Err(String::from("Unexpected carriage return"));
                }
            }
            c if is_identifier_start(c) => {
                // Collect the full identifier.
                let mut identifier = String::new();
                identifier.push(c);

                while let Some(&next_ch) = chars.peek() {
                    if is_identifier_continue(next_ch) {
                        identifier.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Classify keywords, booleans, and identifiers.
                let token = match identifier.as_str() {
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
                };

                tokens.push(token);
            }
            '#' => {
                // Ignore # preserve the new lines
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '\n' || next_ch == '\r' {
                        break;
                    } else {
                        chars.next();
                    }
                }
            }
            '\t' => return Err(String::from("Tabs are not supported; use spaces")),
            ' ' => {}
            _ => return Err(format!("Unexpected character: {ch}")),
        }
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

        let line_tokens =
            tokenize_line(content).map_err(|message| format!("Line {line_number}: {message}"))?;

        for token in &line_tokens {
            match token {
                Token::LeftParen => {
                    opening_lines.push(line_number);
                }

                Token::RightParen => {
                    if opening_lines.pop().is_none() {
                        return Err(format!("Line {line_number}: Unmatched closing parenthesis"));
                    }
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
mod tests {
    use super::*;

    // Check that plus and minus signs produce the correct tokens.
    #[test]
    fn tokenizes_plus_and_minus() {
        let actual = tokenize("+ - +").unwrap();

        let expected = vec![Token::Plus, Token::Minus, Token::Plus];

        assert_eq!(actual, expected);
    }

    // Check that an invalid character returns an error.
    #[test]
    fn reject_invalid_character() {
        let error = tokenize("@").unwrap_err();

        assert_eq!(error, "Line 1: Unexpected character: @");
    }

    // Check that consecutive digits form integer tokens.
    #[test]
    fn tokenizes_integer_literals() {
        let actual = tokenize("123+45").unwrap();

        let expected = vec![Token::Integer(123), Token::Plus, Token::Integer(45)];

        assert_eq!(actual, expected);
    }

    // Check that integers above the i32 limit return an error.
    #[test]
    fn rejects_integer_overflow() {
        let result = tokenize("2147483648");

        assert!(result.is_err());
    }

    // Check arithmetic tokens with extra spaces and leading zeros.
    #[test]
    fn tokenizes_mixed_arithmetic() {
        let actual = tokenize("0  + 12 - 003").unwrap();

        let expected = vec![
            Token::Integer(0),
            Token::Plus,
            Token::Integer(12),
            Token::Minus,
            Token::Integer(3),
        ];

        assert_eq!(actual, expected);
    }

    // Check that empty input produces no tokens.
    #[test]
    fn handles_empty_input() {
        let actual = tokenize("").unwrap();

        assert!(actual.is_empty());
    }

    // Check that input containing only spaces produces no tokens.
    #[test]
    fn handles_spaces_only() {
        let actual = tokenize("     ").unwrap();

        assert!(actual.is_empty());
    }

    // Check that an invalid character after valid tokens still returns an error.
    #[test]
    fn rejects_invalid_character_after_valid_tokens() {
        let error = tokenize("123+@").unwrap_err();

        assert_eq!(error, "Line 1: Unexpected character: @");
    }

    // Check that identifiers can start with a letter or underscore, but not a digit.
    #[test]
    fn recognizes_identifier_start_characters() {
        assert!(is_identifier_start('h'));
        assert!(is_identifier_start('Z'));
        assert!(is_identifier_start('_'));

        assert!(!is_identifier_start('7'));
        assert!(!is_identifier_start('+'));
    }

    // Check that letters, digits, and underscores can continue an identifier.
    #[test]
    fn recognizes_identifier_continuation_characters() {
        assert!(is_identifier_continue('a'));
        assert!(is_identifier_continue('Z'));
        assert!(is_identifier_continue('7'));
        assert!(is_identifier_continue('_'));

        assert!(!is_identifier_continue('+'));
        assert!(!is_identifier_continue(' '));
    }

    // Check that names produce identifier tokens.
    #[test]
    fn tokenizes_identifiers() {
        let actual = tokenize("health player _total").unwrap();

        let expected = vec![
            Token::Identifier(String::from("health")),
            Token::Identifier(String::from("player")),
            Token::Identifier(String::from("_total")),
        ];

        assert_eq!(actual, expected);
    }

    // Check that def and return are recognized as keywords.
    #[test]
    fn tokenizes_keywords() {
        let actual = tokenize("def return health").unwrap();

        let expected = vec![
            Token::Def,
            Token::Return,
            Token::Identifier(String::from("health")),
        ];

        assert_eq!(actual, expected);
    }

    // Check that names starting with def or return remain identifiers.
    #[test]
    fn recognizes_keyword_boundaries() {
        let actual = tokenize("def defense return returnValue player1").unwrap();

        let expected = vec![
            Token::Def,
            Token::Identifier(String::from("defense")),
            Token::Return,
            Token::Identifier(String::from("returnValue")),
            Token::Identifier(String::from("player1")),
        ];

        assert_eq!(actual, expected);
    }

    // Check that parentheses, colons, and commas produce the correct tokens.
    #[test]
    fn tokenizes_punctuation() {
        let actual = tokenize("(:,)").unwrap();

        let expected = vec![
            Token::LeftParen,
            Token::Colon,
            Token::Comma,
            Token::RightParen,
        ];

        assert_eq!(actual, expected);
    }

    // Check that an arrow and a minus sign produce different tokens.
    #[test]
    fn tokenizes_arrow_and_minus() {
        let actual = tokenize("-> -").unwrap();

        let expected = vec![Token::Arrow, Token::Minus];

        assert_eq!(actual, expected);
    }

    // Check the tokens for an arrow followed by a return type.
    #[test]
    fn tokenizes_return_annotation() {
        let actual = tokenize("add->int").unwrap();

        let expected = vec![
            Token::Identifier(String::from("add")),
            Token::Arrow,
            Token::Identifier(String::from("int")),
        ];

        assert_eq!(actual, expected);
    }

    // Check that a single equals sign produces an assignment token.
    #[test]
    fn tokenizes_assignment() {
        let actual = tokenize("health=100").unwrap();

        let expected = vec![
            Token::Identifier(String::from("health")),
            Token::Equal,
            Token::Integer(100),
        ];

        assert_eq!(actual, expected);
    }

    // Check that two equals signs produce an equality token.
    #[test]
    fn tokenizes_equality() {
        let actual = tokenize("health==100").unwrap();

        let expected = vec![
            Token::Identifier(String::from("health")),
            Token::EqualEqual,
            Token::Integer(100),
        ];

        assert_eq!(actual, expected);
    }

    // Check that < and <= produce the correct comparison tokens.
    #[test]
    fn tokenizes_less_comparisons() {
        let actual = tokenize("a<10 b<=20").unwrap();

        let expected = vec![
            Token::Identifier(String::from("a")),
            Token::Less,
            Token::Integer(10),
            Token::Identifier(String::from("b")),
            Token::LessEqual,
            Token::Integer(20),
        ];

        assert_eq!(actual, expected);
    }

    // Check that > and >= produce the correct comparison tokens.
    #[test]
    fn tokenizes_greater_comparisons() {
        let actual = tokenize("a>10 b>=20").unwrap();

        let expected = vec![
            Token::Identifier(String::from("a")),
            Token::Greater,
            Token::Integer(10),
            Token::Identifier(String::from("b")),
            Token::GreaterEqual,
            Token::Integer(20),
        ];

        assert_eq!(actual, expected);
    }

    // Check that != produces an inequality token.
    #[test]
    fn tokenizes_inequality() {
        let actual = tokenize("health!=100").unwrap();

        let expected = vec![
            Token::Identifier(String::from("health")),
            Token::NotEqual,
            Token::Integer(100),
        ];

        assert_eq!(actual, expected);
    }

    // Check that an exclamation mark without an equals sign returns an error.
    #[test]
    fn rejects_standalone_exclamation() {
        let error = tokenize("!").unwrap_err();

        assert_eq!(error, "Line 1: Unexpected character: !");
    }

    // Check that if, else, and while are recognized as keywords.
    #[test]
    fn tokenizes_control_flow_keywords() {
        let actual = tokenize("if health else while").unwrap();

        let expected = vec![
            Token::If,
            Token::Identifier(String::from("health")),
            Token::Else,
            Token::While,
        ];

        assert_eq!(actual, expected);
    }

    // Check that names starting with if, else, or while remain identifiers.
    #[test]
    fn recognizes_control_flow_keyword_boundaries() {
        let actual = tokenize("if iffy elsewhere while_loop").unwrap();

        let expected = vec![
            Token::If,
            Token::Identifier(String::from("iffy")),
            Token::Identifier(String::from("elsewhere")),
            Token::Identifier(String::from("while_loop")),
        ];

        assert_eq!(actual, expected);
    }

    // Check that and, or, and not are recognized as keywords.
    #[test]
    fn tokenizes_logical_keywords() {
        let actual = tokenize("and or not").unwrap();

        let expected = vec![Token::And, Token::Or, Token::Not];

        assert_eq!(actual, expected);
    }

    // Check that names beginning with logical keywords remain identifiers.
    #[test]
    fn recognizes_logical_keyword_boundaries() {
        let actual = tokenize("android origin noteworthy").unwrap();

        let expected = vec![
            Token::Identifier(String::from("android")),
            Token::Identifier(String::from("origin")),
            Token::Identifier(String::from("noteworthy")),
        ];

        assert_eq!(actual, expected);
    }

    // Check that True and False produce boolean tokens.
    #[test]
    fn tokenizes_boolean_literals() {
        let actual = tokenize("True False not True").unwrap();

        let expected = vec![
            Token::Boolean(true),
            Token::Boolean(false),
            Token::Not,
            Token::Boolean(true),
        ];

        assert_eq!(actual, expected);
    }

    // Check that longer boolean-like names and lowercase true remain identifiers.
    #[test]
    fn recognizes_boolean_literal_boundaries() {
        let actual = tokenize("TrueValue False_alarm true").unwrap();

        let expected = vec![
            Token::Identifier(String::from("TrueValue")),
            Token::Identifier(String::from("False_alarm")),
            Token::Identifier(String::from("true")),
        ];

        assert_eq!(actual, expected);
    }

    // Check that an asterisk produces a multiplication token within an expression.
    #[test]
    fn tokenizes_multiplication() {
        let actual = tokenize("2*damage+3").unwrap();

        let expected = vec![
            Token::Integer(2),
            Token::Star,
            Token::Identifier(String::from("damage")),
            Token::Plus,
            Token::Integer(3),
        ];

        assert_eq!(actual, expected);
    }

    // Check the tokens for division, floor division, and modulo.
    #[test]
    fn tokenizes_division_operators() {
        let actual = tokenize("7/2 7//2 7%2").unwrap();

        let expected = vec![
            Token::Integer(7),
            Token::Slash,
            Token::Integer(2),
            Token::Integer(7),
            Token::FloorDivide,
            Token::Integer(2),
            Token::Integer(7),
            Token::Percent,
            Token::Integer(2),
        ];

        assert_eq!(actual, expected);
    }

    // Check the tokens for a divmod call with two arguments.
    #[test]
    fn tokenizes_divmod_call_syntax() {
        let actual = tokenize("divmod(7,2)").unwrap();

        let expected = vec![
            Token::Identifier(String::from("divmod")),
            Token::LeftParen,
            Token::Integer(7),
            Token::Comma,
            Token::Integer(2),
            Token::RightParen,
        ];

        assert_eq!(actual, expected);
    }

    // Check that a Unix line ending separates statements with a newline token.
    #[test]
    fn tokenizes_newlines() {
        let actual = tokenize("x=10\ny=20").unwrap();

        let expected = vec![
            Token::Identifier(String::from("x")),
            Token::Equal,
            Token::Integer(10),
            Token::Newline,
            Token::Identifier(String::from("y")),
            Token::Equal,
            Token::Integer(20),
        ];

        assert_eq!(actual, expected);
    }

    // Check that a Windows line ending produces a single newline token.
    #[test]
    fn tokenizes_windows_newlines() {
        let actual = tokenize("x=10\r\ny=20").unwrap();

        let expected = vec![
            Token::Identifier(String::from("x")),
            Token::Equal,
            Token::Integer(10),
            Token::Newline,
            Token::Identifier(String::from("y")),
            Token::Equal,
            Token::Integer(20),
        ];

        assert_eq!(actual, expected);
    }

    // Check the number of spaces at the start of a line, including empty input.
    #[test]
    fn counts_leading_spaces() {
        assert_eq!(count_leading_spaces("    x = 10"), 4);
        assert_eq!(count_leading_spaces("x = 10"), 0);
        assert_eq!(count_leading_spaces("  x = 10"), 2);
        assert_eq!(count_leading_spaces(""), 0);
    }

    // Check that only increased indentation adds a level and an Indent token.
    #[test]
    fn records_indentation_increase() {
        let mut levels: Vec<usize> = vec![0];
        let mut tokens: Vec<Token> = Vec::new();

        record_indent_increase(4, &mut levels, &mut tokens);

        assert_eq!(levels, vec![0, 4]);
        assert_eq!(tokens, vec![Token::Indent]);

        record_indent_increase(4, &mut levels, &mut tokens);

        assert_eq!(levels, vec![0, 4]);
        assert_eq!(tokens, vec![Token::Indent]);
    }

    // Check that only decreased indentation removes levels and adds Dedent tokens.
    #[test]
    fn records_indentation_decrease() {
        let mut levels: Vec<usize> = vec![0, 4, 8];
        let mut tokens: Vec<Token> = Vec::new();

        record_indent_decrease(4, &mut levels, &mut tokens).unwrap();

        assert_eq!(levels, vec![0, 4]);
        assert_eq!(tokens, vec![Token::Dedent]);

        record_indent_decrease(0, &mut levels, &mut tokens).unwrap();

        assert_eq!(levels, vec![0]);
        assert_eq!(tokens, vec![Token::Dedent, Token::Dedent]);

        record_indent_decrease(0, &mut levels, &mut tokens).unwrap();

        assert_eq!(levels, vec![0]);
        assert_eq!(tokens, vec![Token::Dedent, Token::Dedent]);
    }

    // Check that dedenting to an unknown level returns an error without changing state.
    #[test]
    fn rejects_inconsistent_indentation() {
        let mut levels: Vec<usize> = vec![0, 4, 8];
        let mut tokens: Vec<Token> = Vec::new();

        let result = record_indent_decrease(2, &mut levels, &mut tokens);

        assert!(result.is_err());
        assert_eq!(levels, [0, 4, 8]);
        assert!(tokens.is_empty());
    }

    // Check increased, unchanged, and decreased indentation levels and tokens.
    #[test]
    fn processes_indentation_transitions() {
        let mut levels: Vec<usize> = vec![0];
        let mut tokens: Vec<Token> = Vec::new();

        process_indentation(4, &mut levels, &mut tokens).unwrap();

        assert_eq!(levels, vec![0, 4]);
        assert_eq!(tokens, vec![Token::Indent]);

        process_indentation(4, &mut levels, &mut tokens).unwrap();

        assert_eq!(levels, vec![0, 4]);
        assert_eq!(tokens, vec![Token::Indent]);

        process_indentation(0, &mut levels, &mut tokens).unwrap();

        assert_eq!(levels, vec![0]);
        assert_eq!(tokens, vec![Token::Indent, Token::Dedent]);
    }

    // Check that an invalid indentation change leaves levels and tokens unchanged.
    #[test]
    fn rejects_invalid_indentation_transition() {
        let mut levels: Vec<usize> = vec![0, 4, 8];
        let mut tokens: Vec<Token> = Vec::new();

        let result = process_indentation(2, &mut levels, &mut tokens);

        assert!(result.is_err());
        assert_eq!(levels, vec![0, 4, 8]);
        assert!(tokens.is_empty());
    }

    // Check newline, Indent, and Dedent tokens when entering and leaving a block.
    #[test]
    fn tokenizes_multiline_indentation() {
        let source = "if x:\n    y=1\nz=2";

        let actual = tokenize(source).unwrap();

        let expected = vec![
            Token::If,
            Token::Identifier(String::from("x")),
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Identifier(String::from("y")),
            Token::Equal,
            Token::Integer(1),
            Token::Newline,
            Token::Dedent,
            Token::Identifier(String::from("z")),
            Token::Equal,
            Token::Integer(2),
        ];

        assert_eq!(actual, expected);
    }

    // Check that inconsistent indentation across lines returns an error.
    #[test]
    fn rejects_invalid_multiline_indentation() {
        let source = "if x:\n    y=1\n  z=2";

        let result = tokenize(source);

        assert!(result.is_err());
    }

    // Check that blank lines inside a block add no extra tokens.
    #[test]
    fn ignores_blank_lines_in_blocks() {
        let source = "if x:\n    y=1\n\n    z=2\nw=3";

        let actual = tokenize(source).unwrap();

        let expected = vec![
            Token::If,
            Token::Identifier(String::from("x")),
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Identifier(String::from("y")),
            Token::Equal,
            Token::Integer(1),
            Token::Newline,
            Token::Identifier(String::from("z")),
            Token::Equal,
            Token::Integer(2),
            Token::Newline,
            Token::Dedent,
            Token::Identifier(String::from("w")),
            Token::Equal,
            Token::Integer(3),
        ];

        assert_eq!(actual, expected);
    }

    // Check that leading blank lines add no tokens before the first statement.
    #[test]
    fn ignores_leading_blank_lines() {
        let actual = tokenize("\n    \nx=10").unwrap();

        let expected = vec![
            Token::Identifier(String::from("x")),
            Token::Equal,
            Token::Integer(10),
        ];

        assert_eq!(actual, expected);
    }

    // Check that a Dedent token closes an open block at the end of input.
    #[test]
    fn closes_indentation_at_eof() {
        let actual = tokenize("if x:\n    y=1").unwrap();

        let expected = vec![
            Token::If,
            Token::Identifier(String::from("x")),
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Identifier(String::from("y")),
            Token::Equal,
            Token::Integer(1),
            Token::Dedent,
        ];

        assert_eq!(actual, expected);
    }

    // Check that Dedent tokens close all nested blocks at the end of input.
    #[test]
    fn closes_nested_indentation_at_eof() {
        let actual = tokenize("if x:\n    if y:\n        z=1").unwrap();

        assert!(actual.ends_with(&[Token::Dedent, Token::Dedent]));
    }

    // Check that inline comments are ignored while statement newlines are preserved.
    #[test]
    fn ignores_inline_comments() {
        let actual = tokenize("x=1 # note\ny=2").unwrap();

        let expected = vec![
            Token::Identifier(String::from("x")),
            Token::Equal,
            Token::Integer(1),
            Token::Newline,
            Token::Identifier(String::from("y")),
            Token::Equal,
            Token::Integer(2),
        ];

        assert_eq!(actual, expected);
    }

    // Check that lines containing only comments do not affect the tokens.
    #[test]
    fn ignores_comment_only_lines() {
        let with_comment = tokenize("if x:\n    y=1\n    # note\n    z=2\nw=3").unwrap();

        let without_comment = tokenize("if x:\n    y=1\n    z=2\nw=3").unwrap();

        assert_eq!(with_comment, without_comment);
    }

    // Check that a trailing newline does not change the tokens.
    #[test]
    fn handles_terminal_newline() {
        let without = tokenize("if x:\n    y=1").unwrap();
        let with = tokenize("if x:\n    y=1\n").unwrap();

        assert_eq!(without, with);
    }

    // Check that Unix and Windows line endings produce identical tokens.
    #[test]
    fn normalizes_multiline_line_endings() {
        let unix = tokenize("x=1\ny=2\n").unwrap();
        let windows = tokenize("x=1\r\ny=2\r\n").unwrap();

        assert_eq!(unix, windows);
    }

    // Check that tab indentation returns an error with the correct line number.
    #[test]
    fn rejects_tab_indentation() {
        let error = tokenize("if x:\n\ty=1").unwrap_err();

        assert_eq!(error, "Line 2: Tabs are not supported; use spaces");
    }

    // Check that line breaks inside parentheses add no newline or indentation tokens.
    #[test]
    fn tokenizes_parenthesized_multiline_expression() {
        let actual = tokenize("total=(10+\n    20)").unwrap();

        let expected = vec![
            Token::Identifier(String::from("total")),
            Token::Equal,
            Token::LeftParen,
            Token::Integer(10),
            Token::Plus,
            Token::Integer(20),
            Token::RightParen,
        ];

        assert_eq!(actual, expected);
    }

    // Check that wrapped and single-line function signatures produce identical tokens.
    #[test]
    fn tokenizes_multiline_function_signature() {
        let wrapped = tokenize("def add(\n    a:int,\n    b:int\n)->int:\n    return a+b").unwrap();

        let single_line = tokenize("def add(a:int,b:int)->int:\n    return a+b").unwrap();

        assert_eq!(wrapped, single_line);
    }

    // Check that unclosed and unmatched parentheses return errors.
    #[test]
    fn rejects_unbalanced_parentheses() {
        let unclosed = tokenize("total=(10+20");
        let unmatched = tokenize("total=10)");

        assert!(unclosed.is_err());
        assert!(unmatched.is_err());
    }

    // Check that tokenize includes newline, Indent, and Dedent tokens for a block.
    #[test]
    fn public_tokenize_handles_indentation() {
        let actual = tokenize("if x:\n    y=1").unwrap();

        let expected = vec![
            Token::If,
            Token::Identifier(String::from("x")),
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Identifier(String::from("y")),
            Token::Equal,
            Token::Integer(1),
            Token::Dedent,
        ];

        assert_eq!(actual, expected);
    }

    // Check that tokenize reports an unclosed parenthesis with its line number.
    #[test]
    fn public_tokenize_rejects_unclosed_parentheses() {
        let result = tokenize("x=(10+20");

        assert_eq!(result.unwrap_err(), "Line 1: Unclosed parenthesis");
    }

    // Check the error line number after blank lines and comments.
    #[test]
    fn reports_invalid_character_line() {
        let error = tokenize("x=1\n\n# comment\ny=@").unwrap_err();

        assert_eq!(error, "Line 4: Unexpected character: @");
    }

    // Check that an indentation error reports the line number and space count.
    #[test]
    fn reports_inconsistent_indentation_line() {
        let error = tokenize("if x:\n    y=1\n  z=2").unwrap_err();

        assert_eq!(error, "Line 3: Inconsistent indentation: 2 spaces");
    }

    // Check that an unmatched closing parenthesis error reports its line number.
    #[test]
    fn reports_unmatched_closing_parenthesis_line() {
        let error = tokenize("x=1\ny=2\nz=3)").unwrap_err();

        assert_eq!(error, "Line 3: Unmatched closing parenthesis");
    }

    // Check that an unclosed parenthesis error reports the opening line, including nesting.
    #[test]
    fn reports_unclosed_parenthesis_opening_line() {
        let error = tokenize("x=1\nresult=(2+3").unwrap_err();

        assert_eq!(error, "Line 2: Unclosed parenthesis");

        let nested_error = tokenize("x=(\n    y=(1+2)").unwrap_err();

        assert_eq!(nested_error, "Line 1: Unclosed parenthesis");
    }

    // Check that trailing comments and blank lines still allow the final block to close.
    #[test]
    fn handles_eof_after_trailing_comments() {
        let with_comments = tokenize("if x:\n    y=1\n\n    # Last comment\n").unwrap();

        let without_comments = tokenize("if x:\n    y=1").unwrap();

        assert_eq!(with_comments, without_comments);

        assert!(with_comments.ends_with(&[Token::Integer(1), Token::Dedent]));
    }

    // Check that input containing only comments and blank lines produces no tokens.
    #[test]
    fn handles_comments_only_source() {
        let actual = tokenize("# First comment\n    # Second comment\n\n").unwrap();

        assert!(actual.is_empty());
    }
}
