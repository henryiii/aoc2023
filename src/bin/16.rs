/*!
# 2023 Day 16 - Beams and mirrors

<https://adventofcode.com/2023/day/16>

This is currently a bit messy. Some features in Grid would make itgit  much nicer.
I might rewrite this to use some custom traits to make it simpler.

*/

use std::collections::HashMap;

use derive_more::Constructor;
use grid::Grid;
use strum::{EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Constructor, Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
}

impl Position {
    fn step(&self, dir: Direction) -> Option<Self> {
        let newpos = match dir {
            Direction::Up => Self::new(
                self.row.checked_sub(1)?,
                self.col,
                self.max_row,
                self.max_col,
            ),
            Direction::Down => Self::new(self.row + 1, self.col, self.max_row, self.max_col),
            Direction::Left => Self::new(
                self.row,
                self.col.checked_sub(1)?,
                self.max_row,
                self.max_col,
            ),
            Direction::Right => Self::new(self.row, self.col + 1, self.max_row, self.max_col),
        };
        if newpos.row >= self.max_row || newpos.col >= self.max_col {
            None
        } else {
            Some(newpos)
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<(usize, usize)> for Position {
    fn into(self) -> (usize, usize) {
        (self.row, self.col)
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

fn path(
    grid: &Grid<Tiles>,
    pos: &Position,
    dir: Direction,
    energized: &mut HashMap<Direction, Grid<bool>>,
) {
    use Next::{Double, Single};
    let mut pos = *pos;
    let mut dir = dir;
    // loop here
    loop {
        if energized[&dir][pos.into()] {
            break;
        }
        energized.get_mut(&dir).unwrap()[pos.into()] = true;
        let tile: Tiles = grid[pos.into()];
        match tile.next(dir) {
            Single(d) => {
                if let Some(newpos) = pos.step(d) {
                    dir = d;
                    pos = newpos;
                } else {
                    break;
                }
            }
            Double((d1, d2)) => {
                if let Some(newpos) = pos.step(d2) {
                    path(grid, &newpos, d2, energized);
                }
                if let Some(newpos) = pos.step(d1) {
                    dir = d1;
                    pos = newpos;
                } else {
                    break;
                }
            }
        }
    }
}

fn count_energize(grid: &Grid<Tiles>, pos: &Position, dir: Direction) -> usize {
    let mut energized = HashMap::new();
    for dir in Direction::iter() {
        energized.insert(dir, Grid::new(grid.rows(), grid.cols()));
    }
    path(grid, pos, dir, &mut energized);

    let mut total = 0;
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let count = usize::from(energized[&Direction::Up][(y, x)])
                + usize::from(energized[&Direction::Down][(y, x)])
                + usize::from(energized[&Direction::Left][(y, x)])
                + usize::from(energized[&Direction::Right][(y, x)]);
            total += usize::from(count > 0);
        }
    }
    total
}

fn compute1(text: &str) -> usize {
    let grid = parse(text);
    let pos = Position::new(0, 0, grid.rows(), grid.cols());
    let dir = Direction::Right;
    count_energize(&grid, &pos, dir)
}

fn compute2(text: &str) -> usize {
    let grid = parse(text);
    let mut max = 0;
    for i in 0..(grid.rows()) {
        max = max.max(count_energize(
            &grid,
            &Position::new(i, 0, grid.rows(), grid.cols()),
            Direction::Right,
        ));
        max = max.max(count_energize(
            &grid,
            &Position::new(i, grid.rows() - 1, grid.rows(), grid.cols()),
            Direction::Left,
        ));
    }
    for i in 0..(grid.cols()) {
        max = max.max(count_energize(
            &grid,
            &Position::new(0, i, grid.rows(), grid.cols()),
            Direction::Down,
        ));
        max = max.max(count_energize(
            &grid,
            &Position::new(grid.cols() - 1, i, grid.rows(), grid.cols()),
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
