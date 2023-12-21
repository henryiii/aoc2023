/*!
# 2023 Day 13: Mirrors

<https://adventofcode.com/2023/day/13>

The solution here makes copies of the blocks. These could be made in-place, but
this is fast enough and simpler.
*/

use grid::Grid;

/// Make a block of bools from a string.
fn make_block(block: &str) -> Grid<bool> {
    block
        .lines()
        .map(|x| x.chars().map(|x| x == '#').collect())
        .collect::<Vec<_>>()
        .into()
}

/// Compare around a y mirror line. If you need to compare around x, transpose
/// the block first.
fn compare_mirror_y(block: &Grid<bool>, val: usize) -> bool {
    let width = usize::min(val, block.rows() - val);
    for y in 0..width {
        for x in 0..block.cols() {
            if block[(val - y - 1, x)] != block[(val + y, x)] {
                return false;
            }
        }
    }
    true
}

/// Find the mirror line. Set skip=0 to not skip any values. Otherwise, skip
/// this value if present.
fn compute_block(block: &Grid<bool>, skip: usize) -> Option<usize> {
    (1..block.rows())
        .filter(|&x| x * 100 != skip)
        .find(|&y| compare_mirror_y(block, y))
        .map(|x| x * 100)
        .or_else(|| {
            let mut copied = block.clone();
            copied.transpose();
            (1..copied.rows())
                .filter(|&x| x != skip)
                .find(|&x| compare_mirror_y(&copied, x))
        })
}

/// This computes smudges and checks each block. It skips the non-smudged
/// result.
fn compute_block_one_smudge(block: &Grid<bool>) -> Option<usize> {
    let skip = compute_block(block, 0).unwrap();
    block
        .indexed_iter()
        .map(|((y, x), _)| {
            let mut ng = block.clone();
            ng[(y, x)] = !ng[(y, x)];
            compute_block(&ng, skip)
        })
        .reduce(Option::or)
        .flatten()
}

/// Compute all the first part.
fn compute(text: &str) -> usize {
    text.split("\n\n")
        .map(|s| compute_block(&make_block(s), 0).expect(s))
        .sum()
}

/// Compute all the second part.
fn compute_one_smudge(text: &str) -> usize {
    text.split("\n\n")
        .map(|s| compute_block_one_smudge(&make_block(s)).expect(s))
        .sum()
}

fn main() {
    let text = std::fs::read_to_string("input/13.txt").unwrap();
    let first_result = compute(&text);
    let second_result = compute_one_smudge(&text);
    println!("First = {first_result}");
    println!("Second = {second_result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn simple() {
        let first_result = compute(INPUT);
        assert_eq!(first_result, 405);
    }

    #[test]
    fn smudged() {
        let second_result = compute_one_smudge(INPUT);
        assert_eq!(second_result, 400);
    }

    #[test]
    fn on_each_simple() {
        let blocks = INPUT.split("\n\n");
        let mut blocks = blocks.map(make_block);
        assert_eq!(compute_block(&blocks.next().unwrap(), 0).unwrap(), 5);
        assert_eq!(compute_block(&blocks.next().unwrap(), 0).unwrap(), 400);
        assert!(blocks.next().is_none());
    }

    #[test]
    fn on_each_smudged() {
        let blocks = INPUT.split("\n\n");
        let mut blocks = blocks.map(make_block);
        assert_eq!(
            compute_block_one_smudge(&blocks.next().unwrap()).unwrap(),
            300
        );
        assert_eq!(
            compute_block_one_smudge(&blocks.next().unwrap()).unwrap(),
            100
        );
        assert!(blocks.next().is_none());
    }
}
