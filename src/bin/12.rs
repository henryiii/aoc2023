#![warn(clippy::all, clippy::pedantic)]
#![feature(test)]

extern crate test;

fn compare_spaces(spaces: &[usize], ops: &[usize], conditions: &str) -> bool {
    let (next, mut value) = spaces
        .iter()
        .zip(ops.iter())
        .fold((0, true), |acc, (space, op)| {
            let next = acc.0 + space + op;
            let value = acc.1
                && conditions.chars().skip(acc.0).take(*space).all(|c| c != '#')
                && conditions
                    .chars()
                    .skip(acc.0 + space)
                    .take(*op)
                    .all(|x| x != '.');
            (next, value)
        });
    value &= conditions.chars().skip(next).all(|x| x != '#');
    value
}

/// Returns true if something was changed, recursive.
fn partition_next(max: usize, vals: &mut [usize]) -> bool {
    if vals.is_empty() {
        return false;
    }
    if vals.len() == 1 {
        if vals[0] == max {
            return false;
        }
        vals[0] = max;
        return true;
    }
    if partition_next(max - vals[0], &mut vals[1..]) {
        true
    } else if vals[0] > 0 {
        vals[0] -= 1;
        vals[1] = max - vals[0];
        vals[2..].fill(0);
        true
    } else {
        false
    }
}

fn cmp_line(conditions: &str, ops: &[usize]) -> usize {
    // This is the maximum consecutive space
    let max_space: usize = conditions.len() - (ops.iter().sum::<usize>());
    let num_spaces = ops.len() + 1;
    let mut spaces = vec![0; num_spaces];
    spaces[0] = max_space;
    let mut count = 0;
    while partition_next(max_space, &mut spaces) {
        if spaces.iter().skip(1).take(spaces.len() - 2).all(|x| *x > 0)
            && compare_spaces(&spaces, ops, conditions)
        {
            count += 1;
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
