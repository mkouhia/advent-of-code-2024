use std::io::{self, Read};

/// Day 4: Ceres Search
///
/// <https://adventofcode.com/2024/day/4>
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

/// Find 'XMAS' strings in input in any cardinal or diagonal direction
///
/// Returns number of matches.
fn part1(input: &str) -> u32 {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let dirs: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (-1, -1),
        (1, 1),
    ];
    let mut matches = 0;
    for i0 in 0..chars.len() {
        for j0 in 0..chars[0].len() {
            'dir_search: for (di, dj) in &dirs {
                let i_end = (i0 as i32) + di * 3;
                let j_end = (j0 as i32) + dj * 3;
                if i_end < 0
                    || j_end < 0
                    || i_end as usize >= chars.len()
                    || j_end as usize >= chars[0].len()
                {
                    continue;
                };
                for (k, mc) in ['X', 'M', 'A', 'S'].iter().enumerate() {
                    let i = ((i0 as i32) + di * (k as i32)) as usize;
                    let j = ((j0 as i32) + dj * (k as i32)) as usize;
                    if chars[i][j] != *mc {
                        continue 'dir_search;
                    };
                }
                matches += 1;
            }
        }
    }
    matches
}

/// Find X-MAS patterns in input, return number of matches.
fn part2(input: &str) -> u32 {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let kernel = vec![
        vec!['M', '.', 'S'],
        vec!['.', 'A', '.'],
        vec!['M', '.', 'S'],
    ];
    let mut kernels = vec![kernel];
    for _ in 0..3 {
        kernels.push(rotated(kernels.last().unwrap()));
    }

    let mut matches = 0;
    for kernel in &kernels {
        for i0 in 0..chars.len() - 3 + 1 {
            for j0 in 0..chars[0].len() - 3 + 1 {
                if slice_matches(&chars, i0, j0, (3, 3), kernel) {
                    matches += 1;
                }
            }
        }
    }
    matches
}

/// Does the slice match?
fn slice_matches(
    arr: &[Vec<char>],
    i0: usize,
    j0: usize,
    size: (usize, usize),
    kernel: &[Vec<char>],
) -> bool {
    for i in 0..size.0 {
        for j in 0..size.1 {
            if kernel[i][j] == '.' {
                continue;
            };
            if arr[i + i0][j + j0] != kernel[i][j] {
                return false;
            }
        }
    }
    true
}

/// Rotate 2D array clockwise
fn rotated(arr: &[Vec<char>]) -> Vec<Vec<char>> {
    let new_width = arr.len();
    let new_height = 0..arr[0].len();
    let mut result: Vec<Vec<char>> = (new_height)
        .map(|_| (0..new_width).map(|_| '.').collect())
        .collect();
    for (i, row) in arr.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            result[j][new_width - i - 1] = *item;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE1: &str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    static SAMPLE2: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    static SAMPLE3: &str = "MMMSXXMAS
MSAMXMSMS
AMXSXMAAM
MSAMASMSM
XMASAMXAM
XXAMMXXAM
SMSMSASXS
SAXAMASAA
MAMMMXMMM";

    #[test]
    fn test_rotate() {
        let arr: Vec<Vec<char>> = "
12
34
56"
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
        let arr2 = rotated(&arr);
        let expected: Vec<Vec<char>> = "
531
642"
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
        assert_eq!(arr2, expected);
    }

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(SAMPLE1), 4);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(SAMPLE2), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE2), 9);
    }

    #[test]
    fn test_part2_edges() {
        assert_eq!(part2(SAMPLE3), 9);
    }
}
