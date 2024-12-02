use num_traits::sign::signum;
use std::io::{self, Read};

/// Day 2: Red-Nosed Reports
///
/// <https://adventofcode.com/2024/day/2>
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn is_safe(levels: &[i32]) -> bool {
    let dir = signum(levels[1] - levels[0]);
    if dir == 0 {
        return false;
    }
    for i in 1..(levels.len()) {
        let diff = levels[i] - levels[i - 1];
        if signum(diff) != dir || diff.abs() > 3 {
            return false;
        }
    }
    true
}

fn is_safe_dampened(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }
    for i in 0..levels.len() {
        let levels_i: Vec<i32> = levels
            .iter()
            .enumerate()
            .filter_map(|(j, val)| if i == j { None } else { Some(*val) })
            .collect();
        if is_safe(&levels_i) {
            return true;
        }
    }
    false
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|val| val.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .filter_map(|levels| if is_safe(&levels) { Some(1) } else { None })
        .count() as u32
}
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|val| val.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .filter_map(|levels| {
            if is_safe_dampened(&levels) {
                Some(1)
            } else {
                None
            }
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 2);
    }

    #[test]
    fn is_safe_decreasing() {
        assert!(is_safe(&[7, 6, 4, 2, 1]))
    }

    #[test]
    fn unsafe_increasing_jump() {
        assert!(!is_safe(&[1, 2, 7, 8, 9]))
    }

    #[test]
    fn unsafe_decreasing_jump() {
        assert!(!is_safe(&[9, 7, 6, 2, 1]))
    }

    #[test]
    fn unsafe_nonmonotonic() {
        assert!(!is_safe(&[1, 3, 2, 4, 5]))
    }

    #[test]
    fn unsafe_not_strictly_monotonic() {
        assert!(!is_safe(&[8, 6, 4, 4, 1]))
    }

    #[test]
    fn is_safe_increasing() {
        assert!(is_safe(&[1, 3, 6, 7, 9]))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 4);
    }

    /// Safe by removing the second level, 3.
    #[test]
    fn is_safe_dampened_3() {
        assert!(is_safe_dampened(&[1, 3, 2, 4, 5]))
    }

    /// Safe by removing the third level, 4.
    #[test]
    fn is_safe_dampened_4() {
        assert!(is_safe_dampened(&[8, 6, 4, 4, 1]))
    }

    /// Safe by removing the first level.
    #[test]
    fn is_safe_dampened_1() {
        assert!(is_safe_dampened(&[1, 9, 8, 7, 6]))
    }

    /// Safe by removing the last level.
    #[test]
    fn is_safe_dampened_5() {
        assert!(is_safe_dampened(&[9, 8, 7, 6, 12]))
    }

    /// Catch: not strictly monotonic
    #[test]
    fn is_unsafe_dampened() {
        assert!(!is_safe_dampened(&[86, 86, 84, 86, 86]))
    }
}
