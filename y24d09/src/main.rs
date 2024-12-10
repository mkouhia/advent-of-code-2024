use std::{
    fmt::Display,
    io::{self, Read},
};

/// Day 9: Disk Fragmenter
///
/// <https://adventofcode.com/2024/day/9>
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct Disk(Vec<Option<usize>>);

#[derive(Debug)]
struct FilePosition {
    file_id: usize,
    start: usize,
    len: usize,
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = Vec::new();
        for block in self.0.iter() {
            let s = block
                .map(|id_| {
                    let s = id_.to_string();
                    if s.len() == 1 {
                        s
                    } else {
                        format!("[{}]", s)
                    }
                })
                .unwrap_or_else(|| ".".to_string());
            result.push(s);
        }
        write!(f, "{}", result.join(""))
    }
}

impl Disk {
    fn write(&mut self, file_id: Option<usize>, size: usize) {
        self.0.extend((0..size).map(|_| file_id));
    }

    fn from_dense(input: &str) -> Self {
        let disk_map: Vec<usize> = input
            .chars()
            .map(|c| c.to_digit(10u32).expect("faulty disk") as usize)
            .collect();
        let mut disk = Disk(Vec::with_capacity(disk_map.iter().sum()));
        let mut is_file = true;
        let mut file_id = 0;
        for n_blocks in disk_map {
            let b = if is_file {
                let f = Some(file_id);
                file_id += 1;
                f
            } else {
                None
            };
            disk.write(b, n_blocks);
            is_file = !is_file;
        }
        disk
    }

    /// Move individual blocks from the end to empty spaces in the front.
    fn defragment_part1(&mut self) {
        let mut end_ptr = self.0.len() - 1;
        for i in 0..self.0.len() {
            if (i < end_ptr) & (self.0[i].is_none()) {
                while self.0[end_ptr].is_none() {
                    end_ptr -= 1;
                }
                self.0[i] = self.0[end_ptr];
                self.0[end_ptr] = None;
                end_ptr -= 1;
                // println!("{}", self);
            }
        }
    }

    fn previous_file(&self, end_loc: usize, match_id: Option<usize>) -> Option<FilePosition> {
        let res = self.0[..end_loc]
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(i, v)| {
                v.and_then(|v| {
                    if match_id.is_none_or(|m| m == v) {
                        Some((i + 1, v))
                    } else {
                        None
                    }
                })
            })
            .next();
        if res.is_none() {
            return None;
        }
        let (end, file_id) = res.unwrap();

        let len = self.0[..end]
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(i, v)| if v != &Some(file_id) { Some(i) } else { None })
            .next()
            .unwrap_or(end);

        Some(FilePosition {
            file_id,
            start: end - len,
            len,
        })
    }

    /// Move whole files from the end to empty spaces in the front.
    fn defragment_part2(&mut self) {
        let mut end_pos = self.0.len();
        let mut match_id = None;
        loop {
            let file_position = self.previous_file(end_pos, match_id);

            if file_position.is_none() {
                break;
            }
            let fp = file_position.unwrap();
            if fp.file_id == 0 {
                break;
            }

            for i in 0..(fp.start - fp.len + 1) {
                if self.0[i..(i + fp.len)].iter().all(|v| v.is_none()) {
                    for j in 0..fp.len {
                        self.0[i + j] = self.0[fp.start + j];
                        self.0[fp.start + j] = None;
                    }

                    break;
                }
            }
            end_pos = fp.start;
            match_id = fp.file_id.checked_sub(1);
        }
    }

    fn filesystem_checksum(&self) -> u64 {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, f)| f.and_then(|v| Some((i * v) as u64)))
            .sum()
    }
}

fn part1(input: &str) -> u64 {
    let mut disk = Disk::from_dense(input.trim());
    disk.defragment_part1();
    disk.filesystem_checksum()
}
fn part2(input: &str) -> u64 {
    let mut disk = Disk::from_dense(input.trim());
    disk.defragment_part2();
    disk.filesystem_checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2858);
    }
}
