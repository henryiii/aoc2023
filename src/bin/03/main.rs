use std::io::prelude::*;

struct NumberGrid {
    chars: Vec<(usize, usize)>,
    numbers: Vec<(usize, usize, usize, u32)>,
}

impl NumberGrid {
    fn from_lines(lines: impl Iterator<Item = String>) -> NumberGrid {
        let mut numbers = Vec::new();
        let mut chars = Vec::new();
        for (y, line) in lines.enumerate() {
            let bytes = line.as_bytes();
            for (x, c) in bytes.iter().enumerate() {
                if !c.is_ascii_digit() && *c != b'.' {
                    chars.push((y, x));
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

        NumberGrid { chars, numbers }
    }

    fn filtered_numbers(&self) -> Vec<&(usize, usize, usize, u32)> {
        self.numbers
            .iter()
            .filter(|(y, x, sz, _)| {
                self.chars.iter().any(|(cy, cx)| {
                    (*y == *cy || *y == *cy + 1 || *y + 1 == *cy)
                        && *x <= *cx + 1
                        && *cx < *x + 1 + *sz
                })
            })
            .collect()
    }
}

fn main() {
    let file = std::fs::File::open("resources/03/input.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let grid = NumberGrid::from_lines(lines.map(|x| x.unwrap()));
    let nums = grid.filtered_numbers();
    let sum: u32 = nums.iter().map(|x| x.3).sum();

    let vals: Vec<u32> = nums.iter().map(|x| x.3).collect();
    println!("Vals: {vals:?}");
    println!("Sum: {sum}");
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
    }
}
