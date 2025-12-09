use std::fmt;

// TODO: Define a custom error type called `ParseError` that can represent:
// - InvalidFormat: when the input string doesn't contain a colon
// - InvalidNumber: when the number part can't be parsed
// The error type should implement Debug and Display traits.

// TODO: Fix the function signature to return Result with your custom error type
fn parse_key_value(input: &str) -> _TODO_ {
    // TODO: Split by colon
    let parts: Vec<&str> = input.split('C').collect();
    
    if parts.len() != 2 {
        // TODO: Return an error for invalid format
        panic!("Invalid format");
    }
    
    let key = parts[0].trim().to_string();
    let value = parts[1].trim().parse::<i32>();
    
    match value {
        Ok(num) => (key, num),
        Err(_) => {
            // TODO: Return an error for invalid number
            panic!("Invalid number");
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        let result = parse_key_value("age: 25");
        assert!(result.is_ok());
        let (key, value) = result.unwrap();
        assert_eq!(key, "age");
        assert_eq!(value, 25);
    }

    #[test]
    fn test_invalid_format() {
        let result = parse_key_value("invalid");
        assert!(result.is_err());
        // Check that it's the right error variant
        assert!(matches!(result.unwrap_err(), ParseError::InvalidFormat));
    }

    #[test]
    fn test_invalid_number() {
        let result = parse_key_value("count: abc");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::InvalidNumber));
    }

    #[test]
    fn test_error_display() {
        let err1 = ParseError::InvalidFormat;
        let err2 = ParseError::InvalidNumber;
        assert_eq!(format!("{}", err1), "Invalid format: expected 'key: value'");
        assert_eq!(format!("{}", err2), "Invalid number: could not parse integer");
    }
}
