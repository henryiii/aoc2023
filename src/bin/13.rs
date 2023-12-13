#![warn(clippy::all, clippy::pedantic)]
use grid::Grid;

fn make_block(block: &str) -> Grid<bool> {
    Grid::from_vec(
        block
            .lines()
            .flat_map(|x| x.chars().map(|x| x == '#'))
            .collect(),
        block.lines().take(1).collect::<Vec<&str>>()[0].len(),
    )
}

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

fn compute_block(block: &Grid<bool>) -> Option<usize> {
    (1..block.rows())
        .find(|&y| compare_mirror_y(block, y))
        .map(|x| x * 100)
        .or_else(|| {
            let mut copied = block.clone();
            copied.transpose();
            (1..copied.rows()).find(|&x| compare_mirror_y(&copied, x))
        })
}

fn compute(text: &str) -> (usize, usize) {
    let res = text
        .split("\n\n")
        .map(|x| compute_block(&make_block(x)).expect(x))
        .sum();
    (res, 0)
}

fn main() {
    let text = std::fs::read_to_string("input/13.txt").unwrap();
    let (first_result, second_result) = compute(&text);
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
    fn test() {
        let (first_result, second_result) = compute(INPUT);
        assert_eq!(first_result, 405);
        assert_eq!(second_result, 0);
    }

    #[test]
    fn on_each() {
        let blocks: Vec<&str> = INPUT.split("\n\n").collect();
        let mut blocks = blocks.into_iter().map(make_block);
        assert_eq!(compute_block(&blocks.next().unwrap()).unwrap(), 5);
        assert_eq!(compute_block(&blocks.next().unwrap()).unwrap(), 400);
        assert!(blocks.next().is_none());
    }

}
