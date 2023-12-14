#!/usr/bin/env cargo -Zscript

use std::io::prelude::*;

const NUMS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn index_to_int(line: &[u8], value: usize) -> Option<u32> {
    let result = line[value] as char;
    match result {
        '0'..='9' => result.to_digit(10),
        'z' | 'o' | 't' | 'f' | 's' | 'e' | 'n' => {
            for (i, num) in NUMS.iter().enumerate() {
                let bnum = num.as_bytes();
                let end = value + bnum.len();
                if line.len() >= end && *bnum == line[value..end] {
                    return Some(u32::try_from(i).unwrap());
                }
            }
            None
        }
        _ => None,
    }
}

fn str_to_pair(line: &str) -> (u32, u32) {
    let bytes_arr = line.as_bytes();
    let mut iter = (0..bytes_arr.len()).filter_map(|item| index_to_int(bytes_arr, item));
    let first = iter.next().unwrap();
    (first, iter.next_back().unwrap_or(first))
}

const fn pair_to_int(pair: (u32, u32)) -> u32 {
    10 * pair.0 + pair.1
}

fn main() {
    let file = std::fs::File::open("input/01.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let sum = lines.fold(0, |x, line| x + pair_to_int(str_to_pair(&line.unwrap())));
    println!("Sum: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01b() {
        let lines = &[
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let sum = lines
            .iter()
            .fold(0, |x, line| x + pair_to_int(str_to_pair(line)));
        assert_eq!(sum, 281);
    }
}
