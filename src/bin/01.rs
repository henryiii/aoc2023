#!/usr/bin/env cargo -Zscript

use std::io::prelude::*;

const fn index_to_int(line: &str, value: usize) -> Option<u32> {
    let result = line.as_bytes()[value] as char;
    result.to_digit(10)
}

fn number_line(line: &str) -> Option<u32> {
    let start = line.find(|c: char| c.is_ascii_digit())?;
    let end = line.rfind(|c: char| c.is_ascii_digit())?;
    Some(10 * index_to_int(line, start)? + index_to_int(line, end)?)
}

fn main() {
    let file = std::fs::File::open("input/01.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let sum = lines.fold(0, |x, line| x + number_line(&line.unwrap()).unwrap());
    println!("Sum: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let lines = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let sum = lines
            .iter()
            .fold(0, |x, line| x + number_line(line).unwrap());
        assert_eq!(sum, 142);
    }
}
