use std::{
    collections::VecDeque,
    io::{self, Read},
};

/// Day 10: Hoof It
///
/// <https://adventofcode.com/2024/day/10>
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

struct Grid {
    cells: Vec<Vec<u8>>,
    shape: (usize, usize),
}

impl Grid {
    fn from_string(input: &str) -> Self {
        let cells: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10u32).expect("number") as u8)
                    .collect()
            })
            .collect();
        let shape = (cells.len(), cells[0].len());
        Self { cells, shape }
    }

    fn neighbors(&self, loc: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut nodes = Vec::with_capacity(4);
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let node = match (
                (loc.0 as i32).checked_add(di),
                (loc.1 as i32).checked_add(dj),
            ) {
                (Some(i1), Some(j1)) => (i1 as usize, j1 as usize),
                _ => continue,
            };
            if (node.0 >= self.shape.0) || (node.1 >= self.shape.1) {
                continue;
            }
            nodes.push(node);
        }
        nodes
    }

    /// Find all zero height cells descending from a 9
    ///
    /// Returns positions of 0 height cells.
    fn dfs_descending(&self, root: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut trailheads = Vec::new();
        let mut visited = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        while let Some(node) = queue.pop_front() {
            if visited.contains(&node) {
                continue;
            }
            visited.push(node.clone());

            let height = self.cells[node.0][node.1];
            if height == 0 {
                trailheads.push(node.clone());
                continue;
            }
            for neighbor in self.neighbors(&node) {
                if self.cells[neighbor.0][neighbor.1] == height - 1 {
                    queue.push_front(neighbor.clone());
                }
            }
        }

        trailheads
    }

    /// Find all distinct paths from 0 to 9 cells
    ///
    /// Returns number of paths.
    fn dfs_ascending(&self, root: &(usize, usize)) -> usize {
        let mut rating = 0;

        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        while let Some(node) = queue.pop_front() {
            let height = self.cells[node.0][node.1];
            if height == 9 {
                rating += 1;
                continue;
            }
            for neighbor in self.neighbors(&node) {
                if self.cells[neighbor.0][neighbor.1] == height + 1 {
                    queue.push_front(neighbor.clone());
                }
            }
        }

        rating
    }
}

/// Returns sum of the scores of all trailheads.
///
/// Score is the number of 9-height positions reachable from that trailhead
/// via a hiking trail.
fn part1(input: &str) -> usize {
    let grid = Grid::from_string(input);

    grid.cells
        .iter()
        .enumerate()
        .map(|(i9, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(j9, height)| {
                    if *height == 9 {
                        let n_trailheads = grid.dfs_descending(&(i9, j9)).iter().count();
                        Some(n_trailheads)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

/// Returns the sum of all trailheads' ratings.
///
/// A trailhead's rating is the number of distinct hiking trails which
/// begin at that trailhead.
fn part2(input: &str) -> usize {
    let grid = Grid::from_string(input);

    grid.cells
        .iter()
        .enumerate()
        .map(|(i0, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(j0, height)| {
                    if *height == 0 {
                        let rating = grid.dfs_ascending(&(i0, j0));
                        Some(rating)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 81);
    }
}
