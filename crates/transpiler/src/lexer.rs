use crate::token::Token;

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    for ch in source.chars() {
        match ch {
            '+' => {
                tokens.push(Token::Plus);
            }
            '-' => {
                tokens.push(Token::Minus);
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
}
