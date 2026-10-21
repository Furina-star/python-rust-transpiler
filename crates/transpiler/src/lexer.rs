use crate::token::Token;

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
            },
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            ' ' => {},
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

        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::Plus,
        ];

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

        let expected = vec![
            Token::Integer(123),
            Token::Plus,
            Token::Integer(45),
        ];

        assert_eq!(actual, expected);
    }

    // Test for integer overflow minimum of i32 or signed 32-bit integer
    #[test]
    fn rejects_integer_overflow() {
        let result = tokenize("2147483648");

        assert!(result.is_err());
    }
}
