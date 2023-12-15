/*!
# 2023 Day 14: Tilting rocks

<https://adventofcode.com/2023/day/14>

Implementing this in a single function with a direction required working with
the grid library quite a bit. I think something like `iter_rows_mut` would have
been helpful, but it worked out pretty nicely. It would also be nicer if grids
could be hashed, as a `HashSet` would be nicer than a Vec; I've contributed that
upstream in a PR.

Continuing to enjoy enums with `strum`'s additions.
*/

use grid::Grid;
use std::str::FromStr;
use strum::IntoEnumIterator;

type Num = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumString, strum::Display)]
enum Map {
    #[strum(serialize = ".")]
    Empty,

    #[strum(serialize = "O")]
    RoundRock,

    #[strum(serialize = "#")]
    CubeRock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn read_data(text: &str) -> Grid<Map> {
    Grid::from_vec(
        text.lines()
            .flat_map(|x| x.chars().map(|c| Map::from_str(&c.to_string()).unwrap()))
            .collect::<Vec<_>>(),
        text.lines().next().unwrap().len(),
    )
}

#[cfg(test)]
fn print_grid(grid: &Grid<Map>) {
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            print!("{}", grid[(y, x)]);
        }
        println!();
    }
}

fn compute_load(grid: &Grid<Map>) -> Num {
    grid.indexed_iter()
        .filter_map(|((y, _), c)| {
            if *c == Map::RoundRock {
                Some(grid.rows() - y)
            } else {
                None
            }
        })
        .sum()
}

fn tilt_cycle(grid: &mut Grid<Map>) {
    for dir in Direction::iter() {
        tilt_dir(grid, dir);
    }
}

fn tilt_dir(grid: &mut Grid<Map>, dir: Direction) {
    use Direction::{East, North, South, West};
    let (outer_range, inner_range) = match dir {
        North | South => (0..grid.rows(), 1..grid.cols()),
        West | East => (0..grid.cols(), 1..grid.rows()),
    };

    for outer in outer_range {
        let line: &mut Vec<&mut Map> = &mut match dir {
            West => grid.iter_row_mut(outer).collect(),
            East => grid.iter_row_mut(outer).rev().collect(),
            North => grid.iter_col_mut(outer).collect(),
            South => grid.iter_col_mut(outer).rev().collect(),
        };

        for y in inner_range.clone() {
            for i in (1..=y).rev() {
                if *line[i - 1] == Map::Empty && *line[i] == Map::RoundRock {
                    *line[i - 1] = Map::RoundRock;
                    *line[i] = Map::Empty;
                } else {
                    break;
                }
            }
        }
    }
}

fn compute(text: &str) -> Num {
    let mut grid = read_data(text);
    println!();
    tilt_dir(&mut grid, Direction::North);
    compute_load(&grid)
}

fn compute_cycles(text: &str, cycles: usize) -> Num {
    let mut grid = read_data(text);
    let mut cache: Vec<Grid<Map>> = Vec::new();
    while !cache.iter().any(|x| *x == grid) {
        cache.push(grid.clone());
        tilt_cycle(&mut grid);
    }
    let cycle_start = cache.iter().position(|x| *x == grid).unwrap();
    let cycle_len = cache.len() - cycle_start;
    grid = cache[(cycles - cycle_start) % cycle_len + cycle_start].clone();
    compute_load(&grid)
}

fn main() {
    let text = std::fs::read_to_string("input/14.txt").unwrap();
    let first_result = compute(&text);
    println!("First = {first_result}");

    let second_result = compute_cycles(&text, 1_000_000_000);
    println!("Second = {second_result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_first() {
        let mut grid = read_data(INPUT);
        print_grid(&grid);
        println!();
        tilt_dir(&mut grid, Direction::North);
        print_grid(&grid);
        let result = compute_load(&grid);
        assert_eq!(result, 136);
    }

    #[test]
    fn test_second() {
        let mut grid = read_data(INPUT);
        print_grid(&grid);
        for dir in Direction::iter() {
            println!("Tilt {dir:?}");
            println!();
            tilt_dir(&mut grid, dir);
            print_grid(&grid);
        }

        let result = compute_cycles(INPUT, 1_000_000_000);
        assert_eq!(result, 64);
    }
}
