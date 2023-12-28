/*!
# 2023 Day 10: Pipe Maze
## Maps

<https://adventofcode.com/2023/day/10>

This was originally done by hand, but was much better using `strum`, `Grid`,
etc.  From the beginning, though, I wanted the pretty color map output. It was
really easy once I started using these crates.

This used to stand-alone, but now uses `Direction` from the (local) `aoc2023` crate.
*/

use derive_more::Constructor;
use grid::Grid;
use std::str::FromStr;
use strum::IntoEnumIterator;

use aoc2023::grid_helper::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumString, strum::Display)]
enum MapChar {
    #[strum(serialize = "S")]
    Start,

    #[strum(serialize = "|", to_string = "│")]
    Vertical,

    #[strum(serialize = "-", to_string = "─")]
    Horizontal,

    #[strum(serialize = "J", to_string = "╯")]
    UpLeft,

    #[strum(serialize = "L", to_string = "╰")]
    UpRight,

    #[strum(serialize = "7", to_string = "╮")]
    DownLeft,

    #[strum(serialize = "F", to_string = "╭")]
    DownRight,

    #[strum(serialize = ".", to_string = "•")]
    Empty,
}

#[derive(Debug, Clone, Constructor)]
struct Cursor {
    y: usize,
    x: usize,
}

impl Cursor {
    #[must_use]
    fn step(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::new(self.y - 1, self.x),
            Direction::Down => Self::new(self.y + 1, self.x),
            Direction::Left => Self::new(self.y, self.x - 1),
            Direction::Right => Self::new(self.y, self.x + 1),
        }
    }

    #[must_use]
    fn peek<'a, T>(&self, grid: &'a Grid<T>, dir: Direction) -> Option<&'a T> {
        match dir {
            Direction::Up => grid.get(self.y.checked_sub(1)?, self.x),
            Direction::Down => grid.get(self.y + 1, self.x),
            Direction::Left => grid.get(self.y, self.x.checked_sub(1)?),
            Direction::Right => grid.get(self.y, self.x + 1),
        }
    }

    #[must_use]
    fn find_start(grid: &Grid<MapChar>) -> Self {
        let ((y, x), _) = grid
            .indexed_iter()
            .find(|(_, c)| **c == MapChar::Start)
            .unwrap();
        Self::new(y, x)
    }

    #[must_use]
    fn compute_start_char(&self, grid: &Grid<MapChar>) -> &MapChar {
        use Direction::{Down, Left, Right, Up};

        assert!(grid[(self.y, self.x)] == MapChar::Start);

        match self.get_from_start(grid) {
            (Up, Down) | (Down, Up) => &MapChar::Vertical,
            (Left, Right) | (Right, Left) => &MapChar::Horizontal,
            (Up, Left) | (Left, Up) => &MapChar::UpLeft,
            (Up, Right) | (Right, Up) => &MapChar::UpRight,
            (Down, Left) | (Left, Down) => &MapChar::DownLeft,
            (Down, Right) | (Right, Down) => &MapChar::DownRight,
            _ => panic!("Invalid start"),
        }
    }

    #[must_use]
    fn get_from_start(&self, grid: &Grid<MapChar>) -> (Direction, Direction) {
        use Direction::{Down, Left, Right, Up};
        use MapChar::{DownLeft, DownRight, Horizontal, Start, UpLeft, UpRight, Vertical};

        assert!(grid[(self.y, self.x)] == Start);

        let mut valid_dirs = Direction::iter().filter(|dir| {
            log::debug!("{self:?} {dir:?}");
            self.peek(grid, *dir).map_or(false, |next| {
                log::debug!("Checking ({next}, {dir:?})");
                matches!(
                    (next, dir),
                    (Vertical, Up | Down)
                        | (Horizontal, Left | Right)
                        | (UpLeft, Right | Down)
                        | (UpRight, Left | Down)
                        | (DownLeft, Right | Up)
                        | (DownRight, Left | Up)
                )
            })
        });
        (valid_dirs.next().unwrap(), valid_dirs.next().unwrap())
    }
    #[must_use]
    fn find_end(
        &self,
        start_direction: Direction,
        grid: &Grid<MapChar>,
        mask: &mut Grid<bool>,
    ) -> usize {
        use Direction::{Down, Left, Right, Up};
        use MapChar::{DownLeft, DownRight, Horizontal, Start, UpLeft, UpRight, Vertical};

        let mut cursor = self.clone();
        let mut current_dir = start_direction;
        let mut next_dir: Direction;
        for i in 0.. {
            mask[(cursor.y, cursor.x)] = true;
            next_dir = if let Some(next) = cursor.peek(grid, current_dir) {
                log::debug!("{i}: {current_dir:?} -> {next}");
                match (next, current_dir) {
                    (Vertical, Up) | (UpLeft, Right) | (UpRight, Left) => Up,
                    (Vertical, Down) | (DownLeft, Right) | (DownRight, Left) => Down,
                    (Horizontal, Left) | (UpLeft, Down) | (DownLeft, Up) => Left,
                    (Horizontal, Right) | (UpRight, Down) | (DownRight, Up) => Right,
                    (Start, _) => return i,
                    _ => panic!("No next direction found"),
                }
            } else {
                panic!("No next direction found");
            };
            cursor = cursor.step(current_dir);
            current_dir = next_dir;
        }
        panic!("No end found");
    }
}

#[must_use]
fn is_inside(grid: &Grid<MapChar>, mask: &Grid<bool>, loc: &(usize, usize)) -> bool {
    use Direction::{Down, Up};
    use MapChar::{DownLeft, DownRight, Horizontal, Start, UpLeft, UpRight, Vertical};
    let (y, min_x) = *loc;
    let mut crossings = 0;
    let mut enter = None;
    for x in (min_x)..(grid.cols()) {
        if mask[(y, x)] {
            let mut c = grid[(y, x)];
            if c == Start {
                c = *Cursor::new(y, x).compute_start_char(grid);
            }
            match (c, enter) {
                (Vertical, _) => crossings += 1,
                (DownRight, None) => enter = Some(Down),
                (UpRight, None) => enter = Some(Up),
                (DownLeft, Some(Down)) | (UpLeft, Some(Up)) => enter = None,
                (DownLeft, Some(Up)) | (UpLeft, Some(Down)) => {
                    enter = None;
                    crossings += 1;
                }
                (Horizontal, Some(_)) => (),
                _ => panic!("Invalid char {c:?} at ({y}, {x}), enter={enter:?}"),
            }
        }
    }
    crossings % 2 == 1
}

fn compute_and_print_grid(strs: &[&str]) -> (usize, usize) {
    let grid: Grid<MapChar> = strs
        .iter()
        .map(|x| {
            x.chars()
                .map(|x| MapChar::from_str(&x.to_string()).unwrap())
                .collect()
        })
        .collect::<Vec<_>>()
        .into();
    let mut mask: Grid<bool> = Grid::new(grid.rows(), grid.cols());
    let mut inside: Grid<bool> = Grid::new(grid.rows(), grid.cols());

    let cursor = Cursor::find_start(&grid);
    let current_dir = cursor.get_from_start(&grid).0;
    let count = cursor.find_end(current_dir, &grid, &mut mask);
    for (y, row) in grid.iter_rows().enumerate() {
        for (x, c) in row.enumerate() {
            if !mask[(y, x)] {
                inside[(y, x)] = is_inside(&grid, &mask, &(y, x));
            }
            let s = c.to_string();
            if mask[(y, x)] {
                print!("\x1b[93m{s}\x1b[0m");
            } else if inside[(y, x)] {
                print!("\x1b[92m{s}\x1b[0m");
            } else {
                print!("{s}");
            }
        }
        println!();
    }

    let internal: usize = inside.iter().map(|x| usize::from(*x)).sum();
    (((count + 1) / 2), internal)
}

fn main() {
    env_logger::init();
    let text = std::fs::read_to_string("input/10.txt").unwrap();
    let grid: Vec<&str> = text.lines().collect();
    let (count, internal) = compute_and_print_grid(&grid);

    println!("Count: {count}");
    println!("Internal: {internal}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const INPUT1: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

    const INPUT2: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_1() {
        let grid: Vec<&str> = INPUT1.lines().collect();
        let (count, _) = compute_and_print_grid(&grid);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_2() {
        let grid: Vec<&str> = INPUT2.lines().collect();
        let (count, _) = compute_and_print_grid(&grid);
        assert_eq!(count, 8);
    }
}
