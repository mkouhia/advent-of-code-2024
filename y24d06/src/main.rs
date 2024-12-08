use itertools::Itertools;
use std::{
    collections::HashSet,
    io::{self, Read},
};

/// Day 6: Guard Gallivant
///
/// Process ASCII map and move actor based on rules.
///
/// <https://adventofcode.com/2024/day/6>
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

/// Calculate number of squares, which the guard has visited.
fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_owned()).collect())
        .collect();
    let shape = (grid.len(), grid[0].len());
    let (pos, dir_i) = find_guard(&grid).expect("Guard not found");

    walk_guard(&pos, dir_i, &shape, &grid)
        .unwrap()
        .1
        .iter()
        .unique()
        .count()
}

/// Perform guard walk
///
/// * `pos`  - Initial position.
/// * `dir_i` - Initial direction, where 0=N, 1=E, 2=S, 3=W.
/// * `shape` - Room dimensions (i, j).
/// * `grid` - Original grid.
///
/// Returns visited squares (i, j), or None if path is looped.
fn walk_guard(
    pos: &(usize, usize),
    dir_i: usize,
    shape: &(usize, usize),
    grid: &[Vec<char>],
) -> Option<(Vec<Vec<[bool; 4]>>, Vec<(usize, usize)>)> {
    let dirs = [
        (-1, 0), // UP
        (0, 1),  // RIGHT
        (1, 0),  // DOWN
        (0, -1), // LEFT
    ];
    let mut dir_i = dir_i.clone();
    let shape = (shape.0 as i32, shape.1 as i32);

    let mut pos = (pos.0 as i32, pos.1 as i32);
    let mut next = pos.clone();
    let mut visited: Vec<Vec<[bool; 4]>> = (0..shape.0)
        .map(|_| (0..shape.1).map(|_| [false, false, false, false]).collect())
        .collect();
    let mut path = Vec::new();
    loop {
        pos = (next.0, next.1);
        path.push((pos.0 as usize, pos.1 as usize));

        if visited[pos.0 as usize][pos.1 as usize][dir_i] {
            // Been there, loop detected.
            return None;
        }
        visited[pos.0 as usize][pos.1 as usize][dir_i] = true;

        next = (pos.0 as i32 + dirs[dir_i].0, pos.1 as i32 + dirs[dir_i].1);

        if (next.0 < 0) || (next.0 >= shape.0) || (next.1 < 0) || (next.1 >= shape.1) {
            break;
        }
        if grid[next.0 as usize][next.1 as usize] == '#' {
            dir_i = (dir_i + 1) % 4;
            next = pos.clone(); // Do not go forward, just turn
        }
    }
    Some((visited, path))
}

/// Get guard position from the grid
///
/// Returns ((i, j), <guard_dir>), where guard_dir 0=N, 1=E, 2=S, 3=W.
fn find_guard(grid: &[Vec<char>]) -> Option<((usize, usize), usize)> {
    let guard_dirs = ['^', '>', 'v', '<'];
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if let Some(p) = guard_dirs.iter().position(|x| x == c) {
                return Some(((i, j), p));
            }
        }
    }
    None
}

/// How many possible blocking positions are there
fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_owned()).collect())
        .collect();
    let shape = (grid.len(), grid[0].len());
    let (pos, dir_i) = find_guard(&grid).expect("Guard not found");

    let mut loop_blocks = HashSet::new();
    let (visited, guard_path) = walk_guard(&pos, dir_i, &shape, &grid).unwrap();
    for next in guard_path[1..].iter().dedup() {
        grid[next.0][next.1] = '#';
        if walk_guard(&pos, dir_i, &shape, &grid).is_none() {
            loop_blocks.insert(next.clone());
        }
        grid[next.0][next.1] = '.';
    }
    loop_blocks.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 41);
    }

    #[test]
    fn multiple_turns() {
        let s = "
..#...
.....#
..^.#."
            .trim();
        assert_eq!(part1(&s), 6);
    }

    /// Expect block to be placed at (0, 3)
    #[test]
    fn block_placement() {
        let s = "
#.....
.....#
^.#..#
....#."
            .trim();
        assert_eq!(part2(&s), 1);
    }

    #[test]
    fn test_find_guard() {
        let grid: Vec<Vec<char>> = SAMPLE
            .lines()
            .map(|line| line.chars().map(|c| c.clone()).collect())
            .collect();
        let guard_pos = find_guard(&grid).unwrap();
        assert_eq!(guard_pos, ((6, 4), 0));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 6);
    }
}
