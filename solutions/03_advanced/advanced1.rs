// Enum to represent rotation direction
#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

// Struct to represent a rotation instruction
#[derive(Debug, PartialEq)]
struct Rotation {
    direction: Direction,
    distance: i32,
}

impl Rotation {
    // Parse a line like "L68" or "R42" into a Rotation
    fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }
        
        let direction = match line.chars().next()? {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return None,
        };
        
        let distance = line[1..].parse::<i32>().ok()?;
        
        Some(Rotation { direction, distance })
    }
}

fn count_zero_visits(input: &str) -> usize {
    let mut pos: i32 = 50;
    let mut zero_count = 0;
    
    for line in input.lines() {
        if let Some(rotation) = Rotation::parse(line) {
            // Calculate the change in position
            let delta = match rotation.direction {
                Direction::Left => -rotation.distance,
                Direction::Right => rotation.distance,
            };
            
            // Update position with wraparound (0-99 range)
            // rem_euclid handles negative numbers correctly
            pos = (pos + delta).rem_euclid(100);
            
            // Count if we land on 0
            if pos == 0 {
                zero_count += 1;
            }
        }
    }
    
    zero_count
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