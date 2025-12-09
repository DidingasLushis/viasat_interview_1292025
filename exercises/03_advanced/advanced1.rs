// TODO: Implement a function that processes dial rotations.
// 
// A dial has numbers 0-99 and starts at position 50.
// Each line of input contains a rotation: 'L' or 'R' followed by a distance.
// - 'L' rotates left (toward lower numbers)
// - 'R' rotates right (toward higher numbers)
// The dial wraps around: 0-1 = 99, and 99+1 = 0.
//
// Count how many times the dial points at 0 after any rotation.
//
// Example: "L68\nR48\nL5" starting at 50:
// - L68: 50 - 68 = -18 -> wraps to 82
// - R48: 82 + 48 = 130 -> wraps to 30... wait, that's wrong!
//
// Think carefully about the wrapping behavior.

fn count_zero_visits(input: &str) -> usize {
    // TODO: Implement this function
    // Hint: Consider using an enum for direction
    // Hint: rem_euclid is useful for wrapping arithmetic
    0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(count_zero_visits(input), 3);
    }

    #[test]
    fn test_single_rotation_hits_zero() {
        let input = "L50";
        assert_eq!(count_zero_visits(input), 1);
    }

    #[test]
    fn test_no_zero_hits() {
        let input = "R10\nL5";
        assert_eq!(count_zero_visits(input), 0);
    }

    #[test]
    fn test_wraparound_right() {
        let input = "R50\nR50";
        // 50 + 50 = 0, then 0 + 50 = 50
        assert_eq!(count_zero_visits(input), 1);
    }

    #[test]
    fn test_wraparound_left() {
        let input = "L51\nL49";
        // 50 - 51 = -1 -> 99, then 99 - 49 = 50
        assert_eq!(count_zero_visits(input), 0);
    }
}
