/*!
# 2023 Day X - ...

<https://adventofcode.com/2023/day/X>

*/

use core::fmt::{Debug, Formatter};
use core::ops::{Add, Sub, SubAssign};
use gcollections::ops::{Intersection, IsEmpty};
use interval::{interval::ToInterval, Interval};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Corner {
    z: usize,
    y: usize,
    x: usize,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Block {
    low: Corner,
    high: Corner,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = if self.low.x == self.high.x {
            format!("{}", self.low.x)
        } else {
            format!("{}-{}", self.low.x, self.high.x)
        };
        let y = if self.low.y == self.high.y {
            format!("{}", self.low.y)
        } else {
            format!("{}-{}", self.low.y, self.high.y)
        };
        let z = if self.low.z == self.high.z {
            format!("{}", self.low.z)
        } else {
            format!("{}-{}", self.low.z, self.high.z)
        };
        write!(f, "({x}, {y}, {z})")
    }
}

impl Add<usize> for Block {
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        self.low.z += rhs;
        self.high.z += rhs;
        self
    }
}

impl Sub<usize> for Block {
    type Output = Self;

    fn sub(mut self, rhs: usize) -> Self::Output {
        self.low.z -= rhs;
        self.high.z -= rhs;
        self
    }
}

impl SubAssign<usize> for Block {
    fn sub_assign(&mut self, rhs: usize) {
        self.low.z -= rhs;
        self.high.z -= rhs;
    }
}

impl Block {
    fn new(corner_1: (usize, usize, usize), corner_2: (usize, usize, usize)) -> Self {
        Self {
            low: Corner {
                z: corner_1.2.min(corner_2.2),
                y: corner_1.1.min(corner_2.1),
                x: corner_1.0.min(corner_2.0),
            },
            high: Corner {
                z: corner_1.2.max(corner_2.2),
                y: corner_1.1.max(corner_2.1),
                x: corner_1.0.max(corner_2.0),
            },
        }
    }

    fn x_interval(&self) -> Interval<usize> {
        (self.low.x, self.high.x).to_interval()
    }

    fn y_interval(&self) -> Interval<usize> {
        (self.low.y, self.high.y).to_interval()
    }

    fn z_interval(&self) -> Interval<usize> {
        (self.low.z, self.high.z).to_interval()
    }

    fn overlaps_xy(&self, block: &Self) -> bool {
        !self
            .x_interval()
            .intersection(&block.x_interval())
            .is_empty()
            && !self
                .y_interval()
                .intersection(&block.y_interval())
                .is_empty()
    }

    fn overlaps_xyz(&self, block: &Self) -> bool {
        self.overlaps_xy(block)
            && !self
                .z_interval()
                .intersection(&block.z_interval())
                .is_empty()
    }

    fn high_point(&self, block: &Self) -> Option<usize> {
        if self.overlaps_xy(block) {
            Some(self.high.z)
        } else {
            None
        }
    }

    fn get_blocks_above<'a>(&self, blocks: &'a [Self]) -> Vec<&'a Self> {
        let one_up = self.clone() + 1;
        blocks
            .iter()
            .filter(|x| *x != self && one_up.overlaps_xyz(x))
            .collect()
    }

    fn count_supports(&self, blocks: &[Self]) -> usize {
        let one_down = self.clone() - 1;
        blocks
            .iter()
            .filter(|x| *x != self)
            .filter(|x| one_down.overlaps_xyz(x))
            .count()
    }
}

fn lower_blocks(blocks: &mut [Block]) {
    blocks[0] -= blocks[0].low.z - 1;
    for i in 1..blocks.len() {
        let (blocks_below, blocks_above) = blocks.split_at_mut(i);
        let block = &mut blocks_above[0];
        let level = blocks_below
            .iter()
            .filter_map(|x| Some(x.high_point(block)? + 1))
            .max()
            .unwrap_or(1);
        *block -= block.low.z - level;
    }
}

fn read(text: &str) -> Vec<Block> {
    text.lines()
        .map(|line| {
            let (a, b) = line.split_once('~').unwrap();
            let corner_1: (usize, usize, usize) = a
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let corner_2: (usize, usize, usize) = b
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Block::new(corner_1, corner_2)
        })
        .collect()
}

fn removeable_blocks(blocks: &[Block]) -> Vec<&Block> {
    blocks
        .iter()
        .filter(|x| {
            let above = x.get_blocks_above(blocks);
            if above.is_empty() {
                return true;
            }
            above.iter().all(|y| y.count_supports(blocks) > 1)
        })
        .collect()
}

fn compute1(text: &str) -> usize {
    let mut blocks = read(text);
    blocks.sort();
    lower_blocks(&mut blocks);
    removeable_blocks(&blocks).len()
}

fn compute2(text: &str) -> usize {
    let mut blocks = read(text);
    blocks.sort();
    lower_blocks(&mut blocks);
    blocks
        .iter()
        .map(|b| {
            let mut new_blocks: Vec<Block> = blocks.iter().filter(|x| *x != b).cloned().collect();
            lower_blocks(&mut new_blocks);
            blocks
                .iter()
                .filter(|x| *x != b)
                .zip(new_blocks.iter())
                .filter(|(x, y)| **x != **y)
                .count()
        })
        .sum()
}

fn main() {
    let text = std::fs::read_to_string("input/22.txt").unwrap();
    let result = compute1(&text);
    println!("First = {result}");

    let result = compute2(&text);
    println!("Second = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn test_parts_1() {
        let mut blocks = read(INPUT);
        assert_eq!(blocks.len(), 7);

        blocks.sort();
        println!("{blocks:?}");
        assert_eq!(blocks[0], Block::new((1, 0, 1), (1, 2, 1)));

        lower_blocks(&mut blocks);
        println!("{blocks:?}");
        assert_eq!(blocks[6], Block::new((1, 1, 5), (1, 1, 6)));
    }

    #[test]
    fn test_compute_1() {
        let result = compute1(INPUT);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_compute_2() {
        let result = compute2(INPUT);
        assert_eq!(result, 7);
    }
}
