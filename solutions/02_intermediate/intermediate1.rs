// Transform a vector into a vector of tuples pairing consecutive elements.
// If the input has an odd number of elements, the last tuple should be padded with 0.
// Example: [1, 2, 3, 4, 5] -> [(1, 2), (3, 4), (5, 0)]
fn pair_with_padding(input: Vec<i32>) -> Vec<(i32, i32)> {
    input
        .chunks(2)  // Split into chunks of 2 elements
        .map(|chunk| {
            // First element always exists in a chunk
            let first = chunk[0];
            // Second element might not exist, so pad with 0
            let second = chunk.get(1).copied().unwrap_or(0);
            (first, second)
        })
        .collect()  // Collect into Vec<(i32, i32)>
}

fn main() {
    // You can optionally experiment here.
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