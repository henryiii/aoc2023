#![warn(clippy::all, clippy::pedantic)]
#![feature(test)]

extern crate test;

use itertools::Itertools;

fn compare_spaces(spaces: &[usize], ops: &[usize], start: &str) -> bool {
    let (next, mut value) = spaces
        .iter()
        .zip(ops.iter())
        .fold((0, true), |acc, (space, op)| {
            let next = acc.0 + space + op;
            let value = acc.1
                && start.chars().skip(acc.0).take(*space).all(|c| c != '#')
                && start
                    .chars()
                    .skip(acc.0 + space)
                    .take(*op)
                    .all(|x| x != '.');
            (next, value)
        });
    value &= start.chars().skip(next).all(|x| x != '#');
    value
}

fn partitions(n: usize, k: usize) -> impl Iterator<Item = Vec<usize>> {
    use std::iter::once;
    let nk1 = i64::try_from(n + k - 1).unwrap();
    (0..nk1).combinations(k - 1).map(move |x| {
        once(&-1)
            .chain(x.iter())
            .zip(x.iter().chain(once(&nk1)))
            .map(|(a, b)| usize::try_from(b - a - 1).unwrap())
            .collect()
    })
}

fn cmp_line(start: &str, ops: &[usize]) -> usize {
    // This is the maximum consecutive space
    let max_space: usize = start.len() - (ops.iter().sum::<usize>());
    let num_spaces = ops.len() + 1;
    partitions(max_space, num_spaces)
        .filter(|spaces| {
            spaces.iter().skip(1).take(spaces.len() - 2).all(|x| *x > 0)
                && compare_spaces(spaces, ops, start)
        })
        .count()
}

fn single_line(text: &str, n: usize) -> usize {
    let mut it = text.split_ascii_whitespace();
    let start = it.next().unwrap();
    let end: Vec<usize> = it
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let start = vec![start; n].join("?");
    let end = end.repeat(n);

    cmp_line(&start, &end[..])
}

fn compute(text: &str, n: usize) -> usize {
    text.lines().map(|x| single_line(x, n)).sum()
}

fn main() {
    let text = std::fs::read_to_string("input/12.txt").unwrap();
    let result = compute(&text, 1);
    println!("First = {result}");
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
    fn on_each() {
        let mut lines = INPUT.lines();
        assert_eq!(single_line(lines.next().unwrap(), 1), 1);
        assert_eq!(single_line(lines.next().unwrap(), 1), 4);
        assert_eq!(single_line(lines.next().unwrap(), 1), 1);
        assert_eq!(single_line(lines.next().unwrap(), 1), 1);
        assert_eq!(single_line(lines.next().unwrap(), 1), 4);
        assert_eq!(single_line(lines.next().unwrap(), 1), 10);
    }

    /*
    #[test]
    fn test_5() {
        let result = compute(INPUT, 5);
        assert_eq!(result, 525_152);
    }
    */

    #[bench]
    fn bench_1(b: &mut test::Bencher) {
        b.iter(|| compute(INPUT, 1));
    }

    #[bench]
    fn bench_2(b: &mut test::Bencher) {
        b.iter(|| compute(INPUT, 2));
    }
}
