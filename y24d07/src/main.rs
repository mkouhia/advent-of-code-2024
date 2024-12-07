use itertools::Itertools;
use std::io::{self, Read};

/// Day 7: Bridge Repair
///
/// <https://adventofcode.com/2024/day/7>
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
enum Op {
    ADD, // +
    MUL, // *
    CON, // ||
}

/// Find lines, whose arithmetics can match.
fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            if let Some((result, parts)) = line.split_once(':') {
                let result = result.parse::<u64>();
                let parts: Vec<u32> = parts
                    .split_whitespace()
                    .filter_map(|p| p.parse().ok())
                    .collect();
                match (result, parts) {
                    (Ok(r), parts) => {
                        if let Some(_ops) = find_ops(r, &[Op::ADD, Op::MUL], &parts) {
                            Some(r as u64)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .sum()
}

fn find_ops(result: u64, ops: &[Op], parts: &[u32]) -> Option<Vec<Op>> {
    let pl: usize = (parts.len() - 1) as usize;
    let perms = itertools::repeat_n(ops, pl).multi_cartesian_product();
    'p: for op_candidate in perms {
        let mut running = parts[0] as u64;
        for (i, op) in op_candidate.iter().enumerate() {
            let op_res = match op {
                Op::ADD => running.checked_add(parts[i + 1] as u64),
                Op::MUL => running.checked_mul(parts[i + 1] as u64),
                Op::CON => running
                    .checked_mul(10_u32.pow(parts[i + 1].ilog10() + 1) as u64)
                    .and_then(|v| v.checked_add(parts[i + 1] as u64)),
            };
            if op_res.is_none_or(|r| r > result) {
                // Too high, do not continue this branch
                continue 'p;
            }
            running = op_res.unwrap();
        }
        if running == result {
            return Some(op_candidate.into_iter().map(|v| v.clone()).collect());
        }
    }
    None
}

/// First match with ADD and MUL, then include CON if did not succeed
fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            if let Some((result, parts)) = line.split_once(':') {
                let result = result.parse::<u64>();
                let parts: Vec<u32> = parts
                    .split_whitespace()
                    .filter_map(|p| p.parse().ok())
                    .collect();
                match (result, parts) {
                    (Ok(r), parts) => {
                        if let Some(_ops) = find_ops(r, &[Op::ADD, Op::MUL], &parts) {
                            Some(r as u64)
                        } else if let Some(_ops) = find_ops(r, &[Op::ADD, Op::MUL, Op::CON], &parts)
                        {
                            Some(r as u64)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 3749);
    }

    #[test]
    fn test_find_ops1() {
        let ops = find_ops(292, &[Op::ADD, Op::MUL], &[11, 6, 16, 20]);
        assert_eq!(ops, Some(vec![Op::ADD, Op::MUL, Op::ADD]));
    }
    #[test]
    fn test_find_ops2() {
        let ops = find_ops(190, &[Op::ADD, Op::MUL], &[10, 19]);
        assert_eq!(ops, Some(vec![Op::MUL]));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 11387);
    }
}
