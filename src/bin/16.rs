/*!
# 2023 Day 16: The Floor Will Be Lava
## Beams and mirrors

<https://adventofcode.com/2023/day/16>

The first version was a bit messy. That version added a trait to the grid that
adds `checked_get` which returns an `Option<&T>`` given a `Position``, with
isize coords. This was added to the `grid`` crate directly with .get, so this
workaround was removed.

It also makes `Direction` support being used like a bitflag, so that the `HashMap`
in the original version is avoided.

This version was standalone, but was converted to use the (local) `aoc2023` crate.
*/

use grid::Grid;
use strum::EnumString;

use aoc2023::grid_helper::{Direction, Position};

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
    text.lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<Tiles>().unwrap())
                .collect()
        })
        .collect::<Vec<_>>()
        .into()
}

fn path(grid: &Grid<Tiles>, pos: &Position, dir: Direction, energized: &mut Grid<u8>) {
    use Next::{Double, Single};
    let mut pos = *pos;
    let mut dir = dir;
    // loop here
    loop {
        if energized[pos] & dir as u8 != 0 {
            break;
        }
        energized[pos] |= dir as u8;
        let tile: Tiles = grid[pos];
        match tile.next(dir) {
            Single(d) => {
                let newpos = pos + d;
                if grid.get(newpos.row(), newpos.col()).is_some() {
                    dir = d;
                    pos = newpos;
                } else {
                    break;
                }
            }
            Double((d1, d2)) => {
                let pos1 = pos + d1;
                let pos2 = pos + d2;
                if grid.get(pos2.row(), pos2.col()).is_some() {
                    path(grid, &pos2, d2, energized);
                }
                if grid.get(pos1.row(), pos1.col()).is_some() {
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
    let pos = Position::new(0, 0);
    let dir = Direction::Right;
    count_energize(&grid, &pos, dir)
}

fn compute2(text: &str) -> usize {
    let grid = parse(text);
    let mut max = 0;
    for i in 0..(isize::try_from(grid.rows()).unwrap()) {
        max = max.max(count_energize(
            &grid,
            &Position::new(i, 0),
            Direction::Right,
        ));
        max = max.max(count_energize(
            &grid,
            &Position::new(i, isize::try_from(grid.rows()).unwrap() - 1),
            Direction::Left,
        ));
    }
    for i in 0..(isize::try_from(grid.cols()).unwrap()) {
        max = max.max(count_energize(&grid, &Position::new(0, i), Direction::Down));
        max = max.max(count_energize(
            &grid,
            &Position::new(isize::try_from(grid.cols()).unwrap() - 1, i),
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
