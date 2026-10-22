use crate::token::Token;

// Helper function that checks if the start of a string is a valid identifier
fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

// Helper function that checks if the continuations character is also valid
fn is_identifier_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    // Lookahead loop where it peeks the next upcoming character
    while let Some(ch) = chars.next() {
        match ch {
            '0'..='9' => {
                // Group and collects ascii digits together
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

                // Convert grouped ascii digits and append it
                let value = digits.parse::<i32>().map_err(|e| e.to_string())?;
                let token = Token::Integer(value);
                tokens.push(token);
            }
            '+' => tokens.push(Token::Plus),
            '-' => {
                // Check if -> can be collected as arrow else minus
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Minus);
                }
            }
            '=' => {
                // Check if it's an equality else assignment
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::EqualEqual);
                } else {
                    tokens.push(Token::Equal);
                }
            }
            '<' => {
                // Check if it's Less than Equal <= else Less than <
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::LessEqual);
                } else {
                    tokens.push(Token::Less);
                }
            }
            '>' => {
                // Check if it's Greater than Equal >= else Greater than >
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::GreaterEqual);
                } else {
                    tokens.push(Token::Greater);
                }
            }
            '!' => {
                // Check if it's Not Equal != else error
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
            c if is_identifier_start(c) => {
                // Maximal munch where it detects a reserved keyword until the next character cannot belong to an identifier
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

                // Append matching keywords else identifiers
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
            ' ' => {}
            _ => return Err(format!("Unexpected character: {ch}")),
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test tokenization for Plus and Minus
    #[test]
    fn tokenizes_plus_and_minus() {
        let actual = tokenize("+ - +").unwrap();

        let expected = vec![Token::Plus, Token::Minus, Token::Plus];

        assert_eq!(actual, expected);
    }

    // Test for invalid characters such as @
    #[test]
    fn reject_invalid_character() {
        let error = tokenize("@").unwrap_err();

        assert_eq!(error, "Unexpected character: @");
    }

    // Test grouped tokenized ascii digits
    #[test]
    fn tokenizes_integer_literals() {
        let actual = tokenize("123+45").unwrap();

        let expected = vec![Token::Integer(123), Token::Plus, Token::Integer(45)];

        assert_eq!(actual, expected);
    }

    // Test for integer overflow, upper bound of i32 or signed 32-bit integer
    #[test]
    fn rejects_integer_overflow() {
        let result = tokenize("2147483648");

        assert!(result.is_err());
    }

    // Test mixed arithmetics
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

    // Test for empty input
    #[test]
    fn handles_empty_input() {
        let actual = tokenize("").unwrap();

        assert!(actual.is_empty());
    }

    // Test whitespaces
    #[test]
    fn handles_spaces_only() {
        let actual = tokenize("     ").unwrap();

        assert!(actual.is_empty());
    }

    // Test invalid character after a valid tokens
    #[test]
    fn rejects_invalid_character_after_valid_tokens() {
        let error = tokenize("123+@").unwrap_err();

        assert_eq!(error, "Unexpected character: @");
    }

    // Test starting characters
    #[test]
    fn recognizes_identifier_start_characters() {
        assert!(is_identifier_start('h'));
        assert!(is_identifier_start('Z'));
        assert!(is_identifier_start('_'));

        assert!(!is_identifier_start('7'));
        assert!(!is_identifier_start('+'));
    }

    // Test the continuation characters
    #[test]
    fn recognizes_identifier_continuation_characters() {
        assert!(is_identifier_continue('a'));
        assert!(is_identifier_continue('Z'));
        assert!(is_identifier_continue('7'));
        assert!(is_identifier_continue('_'));

        assert!(!is_identifier_continue('+'));
        assert!(!is_identifier_continue(' '));
    }

    // Test for identifiers
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

    // Test recognize keyword boundaries
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

    // Test Punctuations
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

    // Test arrows and minus token
    #[test]
    fn tokenizes_arrow_and_minus() {
        let actual = tokenize("-> -").unwrap();

        let expected = vec![Token::Arrow, Token::Minus];

        assert_eq!(actual, expected);
    }

    // Test arrow in a function
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

    // Test the assignment equal(=) operator
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

    // Test the equality(==) operator
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

    // Test Less and LessEqual operators
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

    // Test Greater and GreaterEqual operators
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

    // Test valid inequality (!=)
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

    // Test invalid standalone exclamation mark(!)
    #[test]
    fn rejects_standalone_exclamation() {
        let error = tokenize("!").unwrap_err();

        assert_eq!(error, "Unexpected character: !");
    }

    // Test recognized control-flow keywords
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

    // Test keyword-prefix mistakes
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

    // Test logical keywords
    #[test]
    fn tokenizes_logical_keywords() {
        let actual = tokenize("and or not").unwrap();

        let expected = vec![Token::And, Token::Or, Token::Not];

        assert_eq!(actual, expected);
    }

    // Test logical keyword boundaries
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

    // Test boolean literals
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

    // Test boolean literal boundaries
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
}
