/*!
# 2023 Day 9: Number Sequence

<https://adventofcode.com/2023/day/9>

This one is very simple and didn't need anything special, just simple recursion.
*/

fn read(input: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    })
}

fn compute_next(data: &[i64]) -> i64 {
    let next_vec: Vec<_> = data.windows(2).map(|x| x[1] - x[0]).collect();
    let next = if next_vec.iter().sum::<i64>() == 0 {
        0
    } else {
        compute_next(&next_vec)
    };
    *data.last().unwrap() + next
}

fn compute_previous(data: &[i64]) -> i64 {
    let next_vec: Vec<_> = data.windows(2).map(|x| x[1] - x[0]).collect();
    let previous = if next_vec.iter().sum::<i64>() == 0 {
        0
    } else {
        compute_previous(&next_vec)
    };
    *data.first().unwrap() - previous
}

fn main() {
    let text = std::fs::read_to_string("input/09.txt").unwrap();
    let data: Vec<_> = read(&text).collect();
    let forward_sum: i64 = data.iter().map(|x| compute_next(x)).sum();
    println!("Total forward: {forward_sum}");
    let backward_sum: i64 = data.iter().map(|x| compute_previous(x)).sum();
    println!("Total backward: {backward_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test() {
        let data: Vec<_> = read(INPUT).collect();
        assert_eq!(data.len(), 3);
        assert_eq!(data[0], vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(compute_next(&data[0]), 18);
        assert_eq!(compute_next(&data[1]), 28);
        assert_eq!(compute_next(&data[2]), 68);
        assert_eq!(compute_previous(&data[2]), 5);
    }
}
