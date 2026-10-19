fn parse_damage(text: &str) -> Result<i32, std::num::ParseIntError> {
    let contents = text.parse::<i32>()?;
    Ok(contents)
}

fn validate_damage(text: &str) -> Result<i32, String> {
    let value = text.parse::<i32>().map_err(|e| e.to_string())?;

    if value < 0 {
        return Err(String::from("Damage cannot be negative"));
    }

    Ok(value)
}

fn main() {
    // Options
    let damage = vec![100, 200, 300];

    let first_lookup = damage.get(1);
    let second_lookup = damage.get(10);

    match first_lookup {
        Some(value) => println!("Damage record found: {}", value),
        None => println!("Damage record not found"),
    }

    match second_lookup {
        Some(value) => println!("Damage record found: {}", value),
        None => println!("Damage record not found"),
    }

    // Results
    let ok = "123".parse::<i32>();
    let err = "invalid".parse::<i32>();

    match ok {
        Ok(value) => println!("Parsed Integer: {value}"),
        Err(error) => println!("Failed to parse integer: {error}"),
    }

    match err {
        Ok(value) => println!("Parsed Integer: {value}"),
        Err(error) => println!("Failed to parse integer: {error}"),
    }

    // ? propagation
    let valid_input = "123";
    match parse_damage(valid_input) {
        Ok(value) => println!("Success! Damage parsed: {}", value),
        Err(error) => println!("Failed as expected! Reason: {}", error),
    }

    let invalid_input = "invalid";
    match parse_damage(invalid_input) {
        Ok(value) => println!("Success! Damage parsed: {}", value),
        Err(error) => println!("Failed as expected! Reason: {}", error),
    }

    // Exercise
    let test_cases = vec!["250", "0", "-25", "invalid"];

    for test in test_cases {
        println!("\nInput: {}", test);
        match validate_damage(test) {
            Ok(value) => println!("Result: Ok({:?})", value),
            Err(error) => println!("Result: Err({:?})", error),
        }
    }
}
