use std::io::{self, Read};

/// Day X: ...
///
/// <https://adventofcode.com/2024/day/X>
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(_input: &str) -> u32 {
    0
}
fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 999);
    }

    #[ignore]
    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 999);
    }
}
