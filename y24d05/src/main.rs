use std::{
    cmp::Ordering,
    io::{self, Read},
};

/// Day 5: Print Queue
///
/// <https://adventofcode.com/2024/day/5>
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

/// Based on page ordering rules, find out which updates are correct.
///
/// Return sum of middle numbers in correct updates.
fn part1(input: &str) -> usize {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let graph = build_graph(rules);

    updates
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .filter(|pages| update_is_correct(&graph, pages))
        .map(|pages| pages[(pages.len()).div_ceil(2) - 1])
        .sum()
}

/// Adjacency list representation of graph
fn build_graph(rules: &str) -> Vec<Vec<bool>> {
    let mut graph = Vec::with_capacity(100);
    let mut max_n = 0;
    for (i, j) in rules.lines().filter_map(|line| {
        line.split_once("|").and_then(|(a, b)| {
            if let (Ok(a), Ok(b)) = (a.parse(), b.parse()) {
                Some((a, b))
            } else {
                None
            }
        })
    }) {
        while graph.len() <= i {
            graph.push(Vec::with_capacity(100));
        }
        while graph[i].len() <= j {
            graph[i].push(false);
        }
        graph[i][j] = true;

        if j > max_n {
            max_n = j;
        }
    }
    if graph.len() > max_n {
        max_n = graph.len()
    }

    // Fill rest of graph up to length
    while graph.len() < max_n {
        graph.push(Vec::with_capacity(max_n));
    }
    for row in graph.iter_mut() {
        while row.len() <= max_n {
            row.push(false);
        }
    }
    graph
}

/// Check if update is correct.
fn update_is_correct(graph: &[Vec<bool>], pages: &[usize]) -> bool {
    for (p_i, i) in pages.iter().enumerate() {
        for j in pages[p_i + 1..].iter() {
            if !graph[*i][*j] {
                return false;
            }
        }
    }
    true
}

/// Based on page ordering rules, sort incorrect updates.
///
/// Return sum of middle numbers in corrected (previously incorrect) updates.
fn part2(input: &str) -> usize {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let graph = build_graph(rules);

    updates
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .filter(|pages| !update_is_correct(&graph, pages))
        .map(|pages| sort_update(&graph, pages))
        .map(|pages| pages[(pages.len()).div_ceil(2) - 1])
        .sum()
}

/// Sort vector in the order specified in graph.
fn sort_update(graph: &[Vec<bool>], mut pages: Vec<usize>) -> Vec<usize> {
    pages.sort_by(|a, b| match graph[*a][*b] {
        true => Ordering::Less,
        false => Ordering::Greater,
    });
    pages
}

#[cfg(test)]
mod tests {
    use crate::build_graph;

    use super::*;

    static SAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 143);
    }

    #[test]
    fn test_build_graph() {
        let rules = "1|3
1|2
2|3";
        let received = build_graph(rules);
        let expected = vec![
            vec![false, false, false, false],
            vec![false, false, true, true],
            vec![false, false, false, true],
        ];
        assert_eq!(received, expected);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 123);
    }

    #[test]
    fn test_sort_update() {
        let (rules, _) = SAMPLE.split_once("\n\n").unwrap();
        let graph = build_graph(rules);

        let received = sort_update(&graph, vec![75, 97, 47, 61, 53]);
        assert_eq!(received, vec![97, 75, 47, 61, 53]);
    }
}
