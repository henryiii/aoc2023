#!/usr/bin/env cargo -Zscript

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
