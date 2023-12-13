#![warn(clippy::all, clippy::pedantic)]
#![feature(test)]

#[cfg(feature = "progressbar")]
use indicatif::ParallelProgressIterator;
#[cfg(feature = "progressbar")]
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

use std::vec;

extern crate test;

fn cmp_line(conditions: &str, ops: &[usize]) -> usize {
    if ops.is_empty() {
        let val = conditions.chars().all(|x| x != '#');
        return val as usize;
    }

    // This is the maximum consecutive space
    let limit_space: usize = conditions.len() - (ops.iter().sum::<usize>() + ops.len() - 1);
    let mut count = 0;

    let max_space = conditions
        .chars()
        .enumerate()
        .find(|(_, v)| *v == '#')
        .map(|(i, _)| i)
        .unwrap_or(conditions.len());
    let max_space = max_space.min(limit_space);

    for space in 0..=max_space {
        let valid = conditions
            .chars()
            .skip(space)
            .take(ops[0])
            .enumerate()
            .all(|(_, c)| c != '.')
            && conditions
                .chars()
                .skip(space + ops[0])
                .next()
                .unwrap_or('.')
                != '#';
        if conditions.chars().skip(space + ops[0]).count() < 2 {
            count += valid as usize;
        } else if valid {
            count += cmp_line(&conditions[space + ops[0] + 1..], &ops[1..]);
        }
    }
    count
}

fn single_line(text: &str, n: usize) -> usize {
    let mut it = text.split_ascii_whitespace();
    let conditions = it.next().unwrap();
    let ops: Vec<usize> = it
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let conditions = vec![conditions; n].join("?");
    let ops = ops.repeat(n);

    cmp_line(&conditions, &ops[..])
}

fn compute(text: &str, n: usize) -> usize {
    let lines: Vec<&str> = text.lines().collect();

    #[cfg(feature = "progressbar")]
    let iter = lines.par_iter().progress_count(lines.len() as u64);
    #[cfg(not(feature = "progressbar"))]
    let iter = lines.iter();
    
    iter.map(|x| single_line(x, n) ).sum()
}

fn main() {
    let text = std::fs::read_to_string("input/12.txt").unwrap();
    let result = compute(&text, 1);
    println!("1x = {result}");
    let result = compute(&text, 5);
    println!("5x = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_1() {
        let result = compute(INPUT, 1);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_individual() {
        assert_eq!(single_line("? 1", 1), 1);
    }

    #[test]
    fn on_each() {
        let mut lines = INPUT.lines();
        assert_eq!(single_line(lines.next().unwrap(), 1), 1);
        assert_eq!(single_line(lines.next().unwrap(), 1), 4);
        assert_eq!(single_line(lines.next().unwrap(), 1), 1);
        assert_eq!(single_line(lines.next().unwrap(), 1), 1);
        assert_eq!(single_line(lines.next().unwrap(), 1), 4);
        assert_eq!(single_line(lines.next().unwrap(), 1), 10);
    }

    #[test]
    fn test_5() {
        let mut lines = INPUT.lines();
        assert_eq!(single_line(lines.next().unwrap(), 5), 1);
        assert_eq!(single_line(lines.next().unwrap(), 5), 16384);
        assert_eq!(single_line(lines.next().unwrap(), 5), 1);
        assert_eq!(single_line(lines.next().unwrap(), 5), 16);
        assert_eq!(single_line(lines.next().unwrap(), 5), 2500);
        assert_eq!(single_line(lines.next().unwrap(), 5), 506250);
    }

    #[bench]
    fn bench_1(b: &mut test::Bencher) {
        b.iter(|| compute(INPUT, 1));
    }

    #[bench]
    fn bench_2(b: &mut test::Bencher) {
        b.iter(|| compute(INPUT, 2));
    }
}
