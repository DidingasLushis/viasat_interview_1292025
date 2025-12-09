// TODO: Define a Builder trait that can be used to implement the builder pattern.
// The trait should have:
// - An associated type `Output` for the type being built
// - A `new()` method that creates a new builder
// - A `build()` method that consumes the builder and returns the Output

// A simple struct to build
#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
    email: Option<String>,
}

// TODO: Define a UserBuilder struct that implements the Builder trait
// The builder should allow setting name (required), age (required), and email (optional)

struct UserBuilder {
    name: Option<String>,
    age: Option<u32>,
    email: Option<String>,
}

// TODO: Implement methods for UserBuilder:
// - name(mut self, name: String) -> Self
// - age(mut self, age: u32) -> Self  
// - email(mut self, email: String) -> Self

// TODO: Implement the Builder trait for UserBuilder
// The build() method should return User if name and age are set,
// or panic with a helpful message if required fields are missing

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_all_fields() {
        let user = UserBuilder::new()
            .name("Alice".to_string())
            .age(30)
            .email("alice@example.com".to_string())
            .build();
        
        assert_eq!(user.name, "Alice");
        assert_eq!(user.age, 30);
        assert_eq!(user.email, Some("alice@example.com".to_string()));
    }

    #[test]
    fn test_builder_without_email() {
        let user = UserBuilder::new()
            .name("Bob".to_string())
            .age(25)
            .build();
        
        assert_eq!(user.name, "Bob");
        assert_eq!(user.age, 25);
        assert_eq!(user.email, None);
    }

    #[test]
    #[should_panic(expected = "name is required")]
    fn test_builder_missing_name() {
        UserBuilder::new()
            .age(20)
            .build();
    }

    #[test]
    #[should_panic(expected = "age is required")]
    fn test_builder_missing_age() {
        UserBuilder::new()
            .name("Charlie".to_string())
            .build();
    }
}
