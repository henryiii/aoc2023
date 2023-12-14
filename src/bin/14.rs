use cached::proc_macro::cached;
use grid::Grid;
use indicatif::ProgressIterator;
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

#[must_use]
fn tilt_dir(grid: &Grid<Map>, dir: Direction) -> Grid<Map> {
    let mut grid = grid.clone();
    tilt_core(&mut grid, dir);
    grid
}

#[must_use]
fn tilt_cycle(grid: &Grid<Map>, cycles: usize) -> Grid<Map> {
    let cols = grid.cols();
    let v = grid.clone().into_vec();
    tilt_inner(v, cols, cycles)
}

#[cached]
fn tilt_inner(grid: Vec<Map>, cols: usize, cycle: usize) -> Grid<Map> {
    let mut grid = Grid::from_vec(grid, cols);
    for dir in Direction::iter().cycle().take(cycle * 4) {
        tilt_core(&mut grid, dir);
    }
    grid
}

fn tilt_core(grid: &mut Grid<Map>, dir: Direction) {
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
    let grid = read_data(text);
    print_grid(&grid);
    println!();
    let grid = tilt_dir(&grid, Direction::North);
    print_grid(&grid);
    compute_load(&grid)
}

fn compute_cycles(text: &str, cycles: usize, group: usize) -> Num {
    let mut grid = read_data(text);
    for _ in (0..cycles / group).progress_count((cycles / group) as u64) {
        grid = tilt_cycle(&grid, group);
    }
    compute_load(&grid)
}

fn main() {
    let text = std::fs::read_to_string("input/14.txt").unwrap();
    let first_result = compute(&text);
    println!("First = {first_result}");

    let second_result = compute_cycles(&text, 1_000_000_000, 1_000);
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
        let grid = read_data(INPUT);
        print_grid(&grid);
        println!();
        let grid = tilt_dir(&grid, Direction::North);
        print_grid(&grid);
        let result = compute_load(&grid);
        assert_eq!(result, 136);
    }

    #[test]
    fn test_second() {
        let grid = read_data(INPUT);
        print_grid(&grid);
        for dir in Direction::iter() {
            println!("Tilt {dir:?}");
            println!();
            let grid = tilt_dir(&grid, dir);
            print_grid(&grid);
        }

        let result = compute_cycles(INPUT, 1_000_000_000, 10_000);
        assert_eq!(result, 64);
    }
}
