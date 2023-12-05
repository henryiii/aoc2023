#!/usr/bin/env cargo -Zscript

use std::io::prelude::*;

fn index_to_int(line: &[u8], value: usize) -> Option<u32> {
    let result = line[value] as char;
    result.to_digit(10)
}

fn str_to_pair(line: &str) -> (u32, u32) {
    let bytes_arr = line.as_bytes();
    let mut iter = (0..bytes_arr.len()).filter_map(|item| index_to_int(bytes_arr, item));
    let first = iter.next().unwrap();
    (first, iter.last().unwrap_or(first))
}

fn pair_to_int(pair: (u32, u32)) -> u32 {
    10 * pair.0 + pair.1
}

fn main() {
    let file = std::fs::File::open("resources/01/input.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let sum = lines.fold(0, |x, line| x + pair_to_int(str_to_pair(&line.unwrap())));
    println!("Sum: {sum}");
}
