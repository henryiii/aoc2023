/*!
# 2023 Day 16 - Beams and mirrors

<https://adventofcode.com/2023/day/16>

The first version was a bit messy. This version adds a trait to the grid that
adds `checked_get` which returns an `Option<&T>`` given a `Position``, with isize
coords. Once/if this gets added to the `grid`` crate, this can be removed.

It also makes `Direction` support being used like a bitflag, so that the `HashMap`
in the original version is avoided.
*/

use core::ops::Add;

use derive_more::Constructor;
use grid::Grid;
use strum::EnumString;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Direction {
    Up = 0x01,
    Down = 0x02,
    Left = 0x04,
    Right = 0x08,
}

#[derive(Debug, Constructor, Copy, Clone)]
struct Position(isize, isize);

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        use Direction::{Down, Left, Right, Up};

        match dir {
            Up => Self(self.0 - 1, self.1),
            Down => Self(self.0 + 1, self.1),
            Left => Self(self.0, self.1 - 1),
            Right => Self(self.0, self.1 + 1),
        }
    }
}

impl TryFrom<Position> for (usize, usize) {
    type Error = std::num::TryFromIntError;

    fn try_from(pos: Position) -> Result<Self, Self::Error> {
        Ok((usize::try_from(pos.0)?, usize::try_from(pos.1)?))
    }
}

trait CheckedGet<T> {
    fn checked_get(&self, pos: Position) -> Option<&T>;
}

impl<T> CheckedGet<T> for Grid<T> {
    fn checked_get(&self, pos: Position) -> Option<&T> {
        let y = usize::try_from(pos.0).ok()?;
        let x = usize::try_from(pos.1).ok()?;
        self.get(y, x)
    }
}

#[derive(Debug)]
enum Next {
    Single(Direction),
    Double((Direction, Direction)),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString)]
enum Tiles {
    #[strum(serialize = ".")]
    Empty,

    #[strum(serialize = "|")]
    VerticalSplitter,

    #[strum(serialize = "-")]
    HorizontalSplitter,

    #[strum(serialize = "/")]
    ForwardDiagonal,

    #[strum(serialize = r"\")]
    BackwardDiagonal,
}

impl Tiles {
    const fn next(self, dir: Direction) -> Next {
        use Direction::{Down, Left, Right, Up};
        use Next::{Double, Single};
        use Tiles::{
            BackwardDiagonal, Empty, ForwardDiagonal, HorizontalSplitter, VerticalSplitter,
        };

        match (self, dir) {
            (Empty, _) | (VerticalSplitter, Up | Down) | (HorizontalSplitter, Left | Right) => {
                Single(dir)
            }
            (VerticalSplitter, Left | Right) => Double((Up, Down)),
            (HorizontalSplitter, Up | Down) => Double((Left, Right)),
            (ForwardDiagonal, Up) | (BackwardDiagonal, Down) => Single(Right),
            (ForwardDiagonal, Right) | (BackwardDiagonal, Left) => Single(Up),
            (ForwardDiagonal, Down) | (BackwardDiagonal, Up) => Single(Left),
            (ForwardDiagonal, Left) | (BackwardDiagonal, Right) => Single(Down),
        }
    }
}

fn parse(text: &str) -> Grid<Tiles> {
    let grid: Vec<Tiles> = text
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<Tiles>().unwrap())
        })
        .collect();

    Grid::from_vec(grid, text.lines().next().unwrap().len())
}

fn path(grid: &Grid<Tiles>, pos: &Position, dir: Direction, energized: &mut Grid<u8>) {
    use Next::{Double, Single};
    let mut pos = *pos;
    let mut dir = dir;
    // loop here
    loop {
        if energized[pos.try_into().unwrap()] & dir as u8 != 0 {
            break;
        }
        energized[pos.try_into().unwrap()] |= dir as u8;
        let tile: Tiles = grid[pos.try_into().unwrap()];
        match tile.next(dir) {
            Single(d) => {
                let newpos = pos + d;
                if grid.checked_get(newpos).is_some() {
                    dir = d;
                    pos = newpos;
                } else {
                    break;
                }
            }
            Double((d1, d2)) => {
                let pos1 = pos + d1;
                let pos2 = pos + d2;
                if grid.checked_get(pos2).is_some() {
                    path(grid, &pos2, d2, energized);
                }
                if grid.checked_get(pos1).is_some() {
                    dir = d1;
                    pos = pos1;
                } else {
                    break;
                }
            }
        }
    }
}

fn count_energize(grid: &Grid<Tiles>, pos: &Position, dir: Direction) -> usize {
    let mut energized = Grid::new(grid.rows(), grid.cols());
    path(grid, pos, dir, &mut energized);

    let mut total = 0;
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            total += usize::from(energized[(y, x)] > 0);
        }
    }
    total
}

fn compute1(text: &str) -> usize {
    let grid = parse(text);
    let pos = Position(0, 0);
    let dir = Direction::Right;
    count_energize(&grid, &pos, dir)
}

fn compute2(text: &str) -> usize {
    let grid = parse(text);
    let mut max = 0;
    for i in 0..(isize::try_from(grid.rows()).unwrap()) {
        max = max.max(count_energize(&grid, &Position(i, 0), Direction::Right));
        max = max.max(count_energize(
            &grid,
            &Position(i, isize::try_from(grid.rows()).unwrap() - 1),
            Direction::Left,
        ));
    }
    for i in 0..(isize::try_from(grid.cols()).unwrap()) {
        max = max.max(count_energize(&grid, &Position(0, i), Direction::Down));
        max = max.max(count_energize(
            &grid,
            &Position(isize::try_from(grid.cols()).unwrap() - 1, i),
            Direction::Up,
        ));
    }
    max
}

fn main() {
    let text = std::fs::read_to_string("input/16.txt").unwrap();
    let result = compute1(&text);
    println!("First = {result}");

    let result = compute2(&text);
    println!("Second = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_first() {
        let result = compute1(INPUT);
        assert_eq!(result, 46);
    }

    #[test]
    fn test_second() {
        let result = compute2(INPUT);
        assert_eq!(result, 51);
    }
}
