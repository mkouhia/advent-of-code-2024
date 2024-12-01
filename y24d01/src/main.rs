use anyhow::{Context, Result};
use std::{
    io::{self, Read},
    iter::zip,
};

/// Day X: Historian Hysteria
///
/// <https://adventofcode.com/2024/day/X>
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (mut l1, mut l2) = parse_to_vecs(&input)?;
    println!("Part 1: {}", part1(&mut l1, &mut l2));
    println!("Part 2: {}", part2(&l1, &l2));
    Ok(())
}

fn parse_to_vecs(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();
    for row in input.trim().split('\n') {
        let (v1, v2) = row.split_once(' ').context("Row did not have two items")?;
        l1.push(v1.trim().parse()?);
        l2.push(v2.trim().parse()?);
    }
    Ok((l1, l2))
}

/// Find differences between elements
fn part1(l1: &mut Vec<i32>, l2: &mut Vec<i32>) -> u32 {
    l1.sort_unstable();
    l2.sort_unstable();

    let mut res = 0;
    for (a, b) in zip(l1, l2) {
        res += (*b - *a).unsigned_abs();
    }
    res
}

/// Calculate similarity score for the lists
fn part2(l1: &[i32], l2: &[i32]) -> i32 {
    let mut score = 0;
    for a in l1 {
        score += a * l2.iter().filter(|b| a == *b).count() as i32;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_vecs() {
        let (l1, l2) = parse_to_vecs(SAMPLE).unwrap();
        assert_eq!(l1, [3, 4, 2, 1, 3, 3]);
        assert_eq!(l2, [4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_part1() {
        let (mut l1, mut l2) = parse_to_vecs(SAMPLE).unwrap();
        assert_eq!(part1(&mut l1, &mut l2), 11);
    }

    #[test]
    fn test_part2() {
        let (l1, l2) = parse_to_vecs(SAMPLE).unwrap();
        assert_eq!(part2(&l1, &l2), 31);
    }
}
