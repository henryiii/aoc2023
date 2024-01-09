/*!
# 2023 Day 22: Sand Slabs
## 3D falling blocks

<https://adventofcode.com/2023/day/22>

First iteration used intervallum, but I reworked this for a cleaner approach and
a custom interval class. I wanted sorting by lower edges and an easy way to get
the minimum bound. Intervallum for some reason doesn't have public access to
`::new()` or `::low()`. Ideally I also wanted to be able to shift an interval, too.

Plotting code for Blender:

```python
import bpy
import bmesh
from mathutils import Matrix
import numpy as np


TXT = """\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"""

def add_object(bm, start, end):
    scale = np.abs(start - end) + 1
    matrix = Matrix.LocRotScale((start + end) / 2, None, scale)
    bmesh.ops.create_cube(bm, size=1.0, matrix=matrix)


bm = bmesh.new()

for line in TXT.splitlines():
    ax, ay, az, bx, by, bz = map(float, line.replace("~", ",").split(","))
    add_object(bm, np.array((ax, ay, az)), np.array((bx, by, bz)))

me = bpy.data.meshes.new("Mesh")
bm.to_mesh(me)
bm.free()

obj = bpy.data.objects.new("Object", me)
bpy.context.collection.objects.link(obj)
```

*/

use core::fmt::{Debug, Formatter};
use core::ops::{AddAssign, SubAssign};
use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
struct Interval {
    low: usize,
    high: usize,
}

impl Interval {
    fn new(a: usize, b: usize) -> Self {
        Self {
            low: a.min(b),
            high: a.max(b),
        }
    }

    const fn intersects(self, rhs: &Self) -> bool {
        self.low > rhs.high || self.high < rhs.low
    }
}

impl Debug for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.low == self.high {
            write!(f, "{}", self.low)
        } else {
            write!(f, "{}-{}", self.low, self.high)
        }
    }
}

impl AddAssign<usize> for Interval {
    fn add_assign(&mut self, rhs: usize) {
        self.low += rhs;
        self.high += rhs;
    }
}

impl SubAssign<usize> for Interval {
    fn sub_assign(&mut self, rhs: usize) {
        self.low -= rhs;
        self.high -= rhs;
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Block {
    z: Interval,
    y: Interval,
    x: Interval,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}

impl Block {
    fn new(corner_1: (usize, usize, usize), corner_2: (usize, usize, usize)) -> Self {
        Self {
            x: Interval::new(corner_1.0, corner_2.0),
            y: Interval::new(corner_1.1, corner_2.1),
            z: Interval::new(corner_1.2, corner_2.2),
        }
    }

    const fn overlaps_xy(&self, block: &Self) -> bool {
        !self.x.intersects(&block.x) && !self.y.intersects(&block.y)
    }

    const fn overlaps_xyz(&self, block: &Self) -> bool {
        self.overlaps_xy(block) && !self.z.intersects(&block.z)
    }

    const fn high_point(&self, block: &Self) -> Option<usize> {
        if self.overlaps_xy(block) {
            Some(self.z.high)
        } else {
            None
        }
    }

    fn get_blocks_above<'a>(&self, blocks: &'a [Self]) -> Vec<&'a Self> {
        let mut one_up = self.clone();
        one_up.z += 1;
        blocks
            .iter()
            .filter(|x| *x != self && one_up.overlaps_xyz(x))
            .collect()
    }

    fn count_supports(&self, blocks: &[Self]) -> usize {
        let mut one_down = self.clone();
        one_down.z -= 1;
        blocks
            .iter()
            .filter(|x| *x != self)
            .filter(|x| one_down.overlaps_xyz(x))
            .count()
    }
}

fn lower_blocks(blocks: &mut [Block]) {
    blocks[0].z -= blocks[0].z.low - 1;
    for i in 1..blocks.len() {
        let (blocks_below, blocks_above) = blocks.split_at_mut(i);
        let block = &mut blocks_above[0];
        let level = blocks_below
            .iter()
            .filter_map(|x| Some(x.high_point(block)? + 1))
            .max()
            .unwrap_or(1);
        block.z -= block.z.low - level;
    }
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
