// Generic Builder trait that can be implemented for any builder pattern
trait Builder {
    type Output;
    
    fn new() -> Self;
    fn build(self) -> Self::Output;
}

// A simple struct to build
#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
    email: Option<String>,
}

// Builder struct for User
struct UserBuilder {
    name: Option<String>,
    age: Option<u32>,
    email: Option<String>,
}

// Implement builder methods for chaining
impl UserBuilder {
    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
    
    fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
}

// Implement the Builder trait for UserBuilder
impl Builder for UserBuilder {
    type Output = User;
    
    fn new() -> Self {
        UserBuilder {
            name: None,
            age: None,
            email: None,
        }
    }
    
    fn build(self) -> Self::Output {
        User {
            name: self.name.expect("name is required"),
            age: self.age.expect("age is required"),
            email: self.email,
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