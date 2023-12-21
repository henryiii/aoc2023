/*!
# 2023 Day 21 - Garden maps

<https://adventofcode.com/2023/day/21>

*/

use grid::Grid;

#[must_use]
fn read(text: &str) -> Grid<i32> {
    text.lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '#' => -2,
                    '.' => -1,
                    'S' => 0,
                    _ => panic!("Unknown char {x}"),
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .into()
}

#[must_use]
fn steps(grid: Grid<i32>, dist: usize) -> Grid<i32> {
    let mut grid = grid;
    for i in 0..i32::try_from(dist).unwrap() {
        let mut new_grid = grid.clone();
        for y in 0..grid.rows() {
            for x in 0..grid.cols() {
                if grid[(y, x)] == i {
                    for (dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let ty = i32::try_from(y).unwrap() + dy;
                        let tx = i32::try_from(x).unwrap() + dx;

                        if let Some(v) = grid.get(ty, tx) {
                            if *v >= -1 {
                                *new_grid.get_mut(ty, tx).unwrap() = i + 1;
                                new_grid[(y, x)] = -1;
                            }
                        }
                    }
                }
            }
        }
        grid = new_grid;
    }
    grid
}

#[must_use]
fn count_locations(grid: &Grid<i32>) -> usize {
    grid.iter().filter(|x| **x > 0).count()
}

fn print_grid(grid: &Grid<i32>) {
    for row in grid.iter_rows() {
        for x in row {
            let c = match x {
                -2 => '#',
                -1 => '.',
                0 => 'S',
                1.. => 'O',
                _ => panic!("Unknown char {x}"),
            };
            print!("{c}");
        }
        println!();
    }
}

#[must_use]
fn compute1(text: &str) -> usize {
    let grid = read(text);
    let grid = steps(grid, 64);
    print_grid(&grid);
    count_locations(&grid)
}

#[allow(dead_code, unused)]
const fn compute2(text: &str) -> usize {
    0
}

fn main() {
    let text = std::fs::read_to_string("input/21.txt").unwrap();
    let result = compute1(&text);
    println!("First = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_first() {
        let grid = read(INPUT);
        let grid = steps(grid, 6);
        print_grid(&grid);
        let result = count_locations(&grid);
        assert_eq!(result, 16);
    }
}
