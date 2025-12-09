use std::fmt;

// Custom error type that represents two possible error conditions
#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidFormat,
    InvalidNumber,
}

// Implement Display trait for user-friendly error messages
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "Invalid format: expected 'key: value'"),
            ParseError::InvalidNumber => write!(f, "Invalid number: could not parse integer"),
        }
    }
}

// Function returns Result with our custom error type
fn parse_key_value(input: &str) -> Result<(String, i32), ParseError> {
    // Split by colon
    let parts: Vec<&str> = input.split(':').collect();
    
    if parts.len() != 2 {
        return Err(ParseError::InvalidFormat);
    }
    
    let key = parts[0].trim().to_string();
    let value = parts[1].trim().parse::<i32>();
    
    match value {
        Ok(num) => Ok((key, num)),
        Err(_) => Err(ParseError::InvalidNumber),
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