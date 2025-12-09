// TODO: Fix the lifetime annotations in this function.
// It should return the string slice with the most characters.
fn longest_of_three(x: &str, y: &str, z: &str) -> &str {
    if x.len() > y.len() && x.len() > z.len() {
        x
    } else if y.len() > z.len() {
        y
    } else {
        z
    }
}

fn main() {
    // TODO: Fix the compiler error by ensuring all string slices live long enough.
    // CONSTRAINT: You cannot remove any { } scope blocks.

    let string1 = String::from("short");
    let result;
    { // Don't move
        let string2 = String::from("medium length");
        { // Don't move
            let string3 = String::from("this is the longest string");
            result = longest_of_three(&string1, &string2, &string3);
        }
    }
    println!("The longest string is '{result}'");
}
