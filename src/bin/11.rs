/*!
# 2023 Day 11: Cosmic Expansion
## Expanding galaxies

<https://adventofcode.com/2023/day/11>

The key trick here is storing the galaxies as (x,y) pairs. That makes part 2 much easier (see
history for the original part 1). Otherwise, it's quite simpler to solve functionally.
*/

fn compute(text: &str, mul: u64) -> u64 {
    assert!(mul > 0);
    let orig_galaxies: Vec<(u64, u64)> = text
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as u64, y as u64))
                } else {
                    None
                }
            })
        })
        .collect();

    let width = *orig_galaxies.iter().map(|(x, _)| x).max().unwrap();
    let height = *orig_galaxies.iter().map(|(_, y)| y).max().unwrap();

    let empty_cols: Vec<u64> = (0..width)
        .filter(|x| !orig_galaxies.iter().any(|(x2, _)| x == x2))
        .collect();
    let empty_rows: Vec<u64> = (0..height)
        .filter(|y| !orig_galaxies.iter().any(|(_, y2)| y == y2))
        .collect();

    let galaxies = orig_galaxies
        .iter()
        .map(|(x, y)| {
            (
                *x + empty_cols.iter().filter(|x2| x > x2).count() as u64 * (mul - 1),
                *y + empty_rows.iter().filter(|y2| y > y2).count() as u64 * (mul - 1),
            )
        })
        .collect::<Vec<_>>();

    let total: u64 = galaxies
        .iter()
        .map(|g1| {
            galaxies
                .iter()
                .filter(|g2| g1 != *g2)
                .map(|g2| g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1))
                .sum::<u64>()
        })
        .sum::<u64>()
        / 2;
    total
}

fn main() {
    let text = std::fs::read_to_string("input/11.txt").unwrap();
    let result = compute(&text, 2);
    println!("Spacing 1 = {result}");
    let result = compute(&text, 1_000_000);
    println!("Spacing 1,000,000 = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test() {
        assert_eq!(compute(INPUT, 2), 374);
        assert_eq!(compute(INPUT, 10), 1030);
        assert_eq!(compute(INPUT, 100), 8410);
    }
}
