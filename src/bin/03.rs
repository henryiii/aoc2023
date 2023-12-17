/*!
# 2023 Day 3 - Number grid

<https://adventofcode.com/2023/day/3>

This creates a grid of numbers and gears, stored as pairs. It uses a buffered
file reader, which reads the file a line at a time (not required, as the file
isn't that large, but interesting). The implementation is mostly in the struct's
impl block. The struct is mostly just for passing values around together,
though.
*/

use std::io::prelude::*;

const fn adjacent(x: usize, y: usize, cx: usize, cy: usize, sz: usize) -> bool {
    (y == cy || y == cy + 1 || y + 1 == cy) && x <= cx + 1 && cx < x + 1 + sz
}

struct NumberGrid {
    chars: Vec<(usize, usize)>,
    gears: Vec<(usize, usize)>,
    numbers: Vec<(usize, usize, usize, u32)>,
}

impl NumberGrid {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut numbers = Vec::new();
        let mut chars = Vec::new();
        let mut gears = Vec::new();
        for (y, line) in lines.enumerate() {
            let bytes = line.as_bytes();
            for (x, c) in bytes.iter().enumerate() {
                if !c.is_ascii_digit() && *c != b'.' {
                    chars.push((y, x));
                    if *c == b'*' {
                        gears.push((y, x));
                    }
                } else if c.is_ascii_digit() && (x == 0 || !bytes[x - 1].is_ascii_digit()) {
                    let mut end = x + 1;
                    while end < bytes.len() && bytes[end].is_ascii_digit() {
                        end += 1;
                    }
                    let num_str = std::str::from_utf8(&bytes[x..end]).unwrap();
                    let num = num_str.parse().unwrap();
                    numbers.push((y, x, end - x, num));
                }
            }
        }

        Self {
            chars,
            gears,
            numbers,
        }
    }

    fn filtered_numbers(&self) -> Vec<&(usize, usize, usize, u32)> {
        self.numbers
            .iter()
            .filter(|(y, x, sz, _)| {
                self.chars
                    .iter()
                    .any(|(cy, cx)| adjacent(*x, *y, *cx, *cy, *sz))
            })
            .collect()
    }

    fn gear_ratios(&self) -> Vec<u32> {
        self.gears
            .iter()
            .filter_map(|(cy, cx)| {
                let numbers: Vec<u32> = self
                    .numbers
                    .iter()
                    .filter_map(|(y, x, sz, num)| {
                        if adjacent(*x, *y, *cx, *cy, *sz) {
                            Some(*num)
                        } else {
                            None
                        }
                    })
                    .collect();
                if numbers.len() == 2 {
                    Some(numbers[0] * numbers[1])
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() {
    let file = std::fs::File::open("input/03.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let grid = NumberGrid::from_lines(lines.map(std::result::Result::unwrap));
    let nums = grid.filtered_numbers();
    let sum: u32 = nums.iter().map(|x| x.3).sum();

    println!("Sum: {sum}");

    let ratios: u32 = grid.gear_ratios().iter().sum();
    println!("Ratios: {ratios:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_03() {
        let lines = INPUT.lines().map(|x| x.to_string());
        let grid = NumberGrid::from_lines(lines);
        assert_eq!(grid.chars.len(), 6);
        assert_eq!(grid.numbers.len(), 10);
        assert!(grid.numbers.contains(&(0, 0, 3, 467)));
        assert!(
            grid.chars.contains(&(3, 6)),
            "Expected (3, 6) to be in {:?}",
            grid.chars
        );
        let nums = grid.filtered_numbers();
        assert_eq!(nums.len(), 8, "Expected 8 numbers, got {:?}", nums);
        let vals: Vec<u32> = nums.iter().map(|x| x.3).collect();
        assert!(!vals.contains(&114));
        assert!(!vals.contains(&58));

        let sum: u32 = nums.iter().map(|x| x.3).sum();
        assert_eq!(sum, 4361);

        let ratios: u32 = grid.gear_ratios().iter().sum();
        assert_eq!(ratios, 467835);
    }
}
