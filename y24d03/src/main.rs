use regex::Regex;
use std::io::{self, Read};

/// Day 3: Mull It Over
///
/// <https://adventofcode.com/2024/day/3>
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

/// Process all `mul(a,b)` instructions.
///
/// Returns sum(a * b) over all a,b pairs.
fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
        })
        .sum()
}

/// Process `mul(a,b)` instructions when enabled.
///
/// As part 1, but when meeting string `do()` set enabled flag
/// and when meeting `don't()`, unset flag. Process only when enabled.
///
/// Returns sum(a * b) over all a,b pairs when enabled.
fn part2(input: &str) -> i32 {
    let re_all = Regex::new(r"(?:mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    let mut enabled = true;
    for (full, []) in re_all.captures_iter(input).map(|c| c.extract()) {
        match full {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ if enabled => {
                let (_, [a, b]) = re_mul.captures(full).unwrap().extract();
                result += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap() as i32;
            }
            _ => {}
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    static SAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE2), 48);
    }
}
