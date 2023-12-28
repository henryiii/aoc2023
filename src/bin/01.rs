#!/usr/bin/env cargo -Zscript
/*!
# 2023 Day 1: Trebuchet?!
##  Number line

<https://adventofcode.com/2023/day/1>

This is a simple introduction to Rust. I'm doing some functional style
processing to find the first and last digits. I originally used a line iterator
(see history), but simplified it to reading the whole file for most examples, as
Advent of Code input will always fit in memory.  I left the original line
iterator for Day 1b (also in Day 3, which is a bit more complex).

To keep this simple as possible, I've put 1b in a separate file. Most days
compute everything in one.

Day 1 is also the only day that supports Rust nightlies' experimental script
mode (`cargo script`).
*/

fn number_line(line: &str) -> u32 {
    let mut chars = line.chars().filter_map(|c| c.to_digit(10));
    let start = chars.next().unwrap();
    let end = chars.last().unwrap_or(start);
    10 * start + end
}

fn main() {
    let text = std::fs::read_to_string("01.txt").unwrap();
    let sum: u32 = text.lines().map(number_line).sum();
    println!("Sum: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchetn";

    #[test]
    fn test_01() {
        let sum: u32 = INPUT.lines().map(number_line).sum();
        assert_eq!(sum, 142);
    }
}
