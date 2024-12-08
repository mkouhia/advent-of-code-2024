use std::{
    collections::HashMap,
    io::{self, Read},
};

use itertools::Itertools;

/// Day 8: Resonant Collinearity
///
/// <https://adventofcode.com/2024/day/8>
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> u32 {
    antinode_count(input, antinode_pos_part1)
}

fn part2(input: &str) -> u32 {
    antinode_count(input, antinode_pos_part2)
}

/// Calculate number of distinct antinodes on the map
fn antinode_count(
    input: &str,
    an_fun: fn(&(usize, usize), &(usize, usize), &(usize, usize)) -> Vec<(usize, usize)>,
) -> u32 {
    let mut antennae: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let shape = (input.lines().count(), input.lines().next().unwrap().len());
    input.lines().enumerate().for_each(|(i, line)| {
        line.char_indices()
            .filter(|(_, c)| *c != '.')
            .for_each(|(j, c)| antennae.entry(c).or_insert(Vec::new()).push((i, j)))
    });
    let mut antinodes: Vec<Vec<u32>> = (0..shape.0)
        .map(|_| (0..shape.1).map(|_| 0).collect())
        .collect();

    for (_, antenna_locs) in antennae.iter() {
        for (a, b) in antenna_locs.iter().tuple_combinations() {
            for (i, j) in an_fun(a, b, &shape) {
                antinodes[i][j] = 1;
            }
        }
    }
    antinodes.iter().map(|line| line.iter().sum::<u32>()).sum()
}

/// Calculate antinode positions: only one at each side
fn antinode_pos_part1(
    a: &(usize, usize),
    b: &(usize, usize),
    shape: &(usize, usize),
) -> Vec<(usize, usize)> {
    let di = b.0 as i32 - a.0 as i32;
    let dj = b.1 as i32 - a.1 as i32;
    [
        (a.0 as i32 - di, a.1 as i32 - dj),
        (b.0 as i32 + di, b.1 as i32 + dj),
    ]
    .into_iter()
    .filter(|(i, j)| (*i >= 0) & (*j >= 0) & ((*i as usize) < shape.0) & ((*j as usize) < shape.1))
    .map(|(i, j)| (i as usize, j as usize))
    .collect()
}

/// Calculate antinode positions: any number at each side
fn antinode_pos_part2(
    a: &(usize, usize),
    b: &(usize, usize),
    shape: &(usize, usize),
) -> Vec<(usize, usize)> {
    let di = b.0 as i32 - a.0 as i32;
    let dj = b.1 as i32 - a.1 as i32;
    let mut pos = vec![a.clone()];
    for dir in [-1, 1] {
        let mut k = 1;
        let mut i = a.0 as i32 + di * k * dir;
        let mut j = a.1 as i32 + dj * k * dir;
        while (i >= 0) & (j >= 0) & ((i as usize) < shape.0) & ((j as usize) < shape.1) {
            pos.push((i as usize, j as usize));
            i = a.0 as i32 + di * k * dir;
            j = a.1 as i32 + dj * k * dir;
            k += 1;
        }
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 14);
    }

    #[test]
    fn test_antinode_pos_a() {
        assert_eq!(
            vec![(1, 3), (7, 6)],
            antinode_pos_part1(&(3, 4), &(5, 5), &(12, 12))
        )
    }

    #[test]
    fn test_antinode_pos_0() {
        assert_eq!(
            vec![(0, 11), (3, 2)],
            antinode_pos_part1(&(1, 8), &(2, 5), &(12, 12))
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 34);
    }
}
