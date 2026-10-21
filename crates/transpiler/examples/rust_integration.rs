// Tokens
enum Token {
    Integer(i32),
    Plus,
}

// Parse string to valid integers
fn build_expression(left: &str, right: &str) -> Result<Vec<Token>, String> {
    let left = left.parse::<i32>().map_err(|e| e.to_string())?;
    let right = right.parse::<i32>().map_err(|e| e.to_string())?;

    let tokens = vec![Token::Integer(left), Token::Plus, Token::Integer(right)];

    Ok(tokens)
}

// Display the Token and its lexeme
fn display_tokens(tokens: &Vec<Token>) {
    for token in tokens.iter() {
        match token {
            Token::Integer(value) => println!("Integer: {}", value),
            Token::Plus => println!("Plus"),
        }
    }
}

fn main() {
    // Input two valid integers
    let result = build_expression("50", "75");

    // Display successful expression or error
    match result {
        Ok(tokens) => {
            display_tokens(&tokens);
            println!("Token count: {}", tokens.len());
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}
