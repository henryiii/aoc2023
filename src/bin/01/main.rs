#!/usr/bin/env cargo -Zscript

use std::io::prelude::*;

fn index_to_int(line: &str, value: usize) -> u32 {
    let result = line.as_bytes()[value] as char;
    result.to_digit(10).unwrap()
}

fn main() {
    let mut sum = 0;
    let file = std::fs::File::open("resources/01/input.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    for _line in lines {
        let line = _line.unwrap();
        let start = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let end = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
        sum += 10 * index_to_int(&line, start) + index_to_int(&line, end);
    }
    println!("Sum: {sum}");
}
