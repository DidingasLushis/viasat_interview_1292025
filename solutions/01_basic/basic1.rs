// This function takes three string slices with the same lifetime 'a
// and returns a reference with that same lifetime 'a.
// This ensures the returned reference is valid as long as all inputs are valid.
fn longest_of_three<'a>(x: &'a str, y: &'a str, z: &'a str) -> &'a str {
    if x.len() > y.len() && x.len() > z.len() {
        x
    } else if y.len() > z.len() {
        y
    } else {
        z
    }
}

fn main() {
    // CONSTRAINT: Cannot remove any { } scope blocks.
    // Solution: Move result assignment outside nested scopes and use println! inside
    // where all three strings are still alive.

    let string1 = String::from("short");
    {
        let string2 = String::from("medium length");
        {
            let string3 = String::from("this is the longest string");
            let result = longest_of_three(&string1, &string2, &string3);
            // Move the println! here where all strings are still in scope
            println!("The longest string is '{result}'");
        }
    }
}