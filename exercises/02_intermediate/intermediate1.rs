// TODO: Fix this function to use iterator methods instead of loops.
// Transform a vector into a vector of tuples pairing consecutive elements.
// If the input has an odd number of elements, the last tuple should be padded with 0.
// Example: [1, 2, 3, 4, 5] -> [(1, 2), (3, 4), (5, 0)]
fn pair_with_padding(input: Vec<i32>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    
    for i in 0..input.len() {
        if i % 2 == 0 {
            let first = input[i];
            let second = if i + 1 < input.len() {
                input[i + 1]
            } else {
                0
            };
            result.push((first, second));
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_length() {
        let input = vec![1, 2, 3, 4];
        let expected = vec![(1, 2), (3, 4)];
        assert_eq!(pair_with_padding(input), expected);
    }

    #[test]
    fn test_odd_length() {
        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![(1, 2), (3, 4), (5, 0)];
        assert_eq!(pair_with_padding(input), expected);
    }

    #[test]
    fn test_single_element() {
        let input = vec![42];
        let expected = vec![(42, 0)];
        assert_eq!(pair_with_padding(input), expected);
    }

    #[test]
    fn test_empty() {
        let input = vec![];
        let expected = vec![];
        assert_eq!(pair_with_padding(input), expected);
    }
}
