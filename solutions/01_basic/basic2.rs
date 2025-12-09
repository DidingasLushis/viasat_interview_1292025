// Define an enum that can hold either a String or an i32.
#[derive(Debug)]
enum Value {
    Text(String),
    Number(i32),
}

// This function takes ownership of the Value since it consumes it.
fn print_value(v: Value) {
    match v {
        Value::Text(s) => println!("Text: {}", s),
        Value::Number(n) => println!("Number: {}", n),
    }
}

// This function only needs to borrow the Value to read its length.
fn get_length(v: &Value) -> usize {
    match v {
        Value::Text(s) => s.len(),
        Value::Number(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_value() {
        let v = Value::Text(String::from("hello"));
        assert_eq!(get_length(&v), 5);
        print_value(v);
    }

    #[test]
    fn test_number_value() {
        let v = Value::Number(42);
        assert_eq!(get_length(&v), 0);
        print_value(v);
    }

    #[test]
    fn test_multiple_uses() {
        let text = Value::Text(String::from("world"));
        let len = get_length(&text);
        print_value(text);
        assert_eq!(len, 5);
    }
}