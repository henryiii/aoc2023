/*!
# 2023 Day 21 - Garden maps

<https://adventofcode.com/2023/day/21>

Part 1 was pretty easy, just avoiding mistakes was tricky. I basically just
do a convolution map [0 1 0; 1 0 1; 0 1 0] by hand. Part 2 was tricky, and
had to look around for a bit of insperation. I basically just expand and
compute the first couple of board-lenths (131), and then solve; the puzzle
boards are all clear around the diagonals and edges, so each expansion adds
the same amount (partially squared) to the total.
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
fn expand(grid: &Grid<i32>) -> Grid<i32> {
    let mut new_grid = Grid::new(grid.rows() * 3, grid.cols() * 3);
    for i in 0..3 {
        for j in 0..3 {
            for ((y, x), v) in grid.indexed_iter() {
                let new_val = if *v == 0 && (i != 1 || j != 1) {
                    -1
                } else {
                    *v
                };
                new_grid[(y + i * grid.rows(), x + j * grid.cols())] = new_val;
            }
        }
    }
    new_grid
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

fn count_block(grid: &Grid<i32>, is_odd: bool) -> usize {
    grid.indexed_iter()
        .filter(|((y, x), v)| **v != -2 && (y + x) % 2 == usize::from(is_odd))
        .count()
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

fn compute2(text: &str) -> usize {
    let grid = read(text);
    count_block(&grid, true)
}

fn main() {
    let text = std::fs::read_to_string("input/21.txt").unwrap();
    let result = compute1(&text);
    println!("First = {result}");

    let result = compute2(&text);
    println!("Full block (odd steps) = {result}");
    let val = 26_501_365;
    println!("Steps = {val}");

    let grid = read(&text);
    let grid = steps(grid, 65);
    let y_0 = count_locations(&grid);
    println!("131*0 = 65 = {y_0}");

    let grid = read(&text);
    let grid = expand(&grid);
    let grid = steps(grid, 65 + 131);
    let y_1 = count_locations(&grid);
    println!("131*1 + 65 = {y_1}");

    let grid = read(&text);
    let grid = expand(&grid);
    let grid = expand(&grid);
    let grid = steps(grid, 65 + 131 * 2);
    let y_2 = count_locations(&grid);
    println!("131*2 + 65 = {y_2}");

    let a2 = y_2 - 2 * y_1 + y_0;
    let b2 = 4 * y_1 - 3 * y_0 - y_2;
    let c = y_0;

    println!("{a2}/2 x^2 +{b2}/2 x + {c} = y");
    println!("x=0, y={c}");
    println!("x=1, y={}", (a2 + b2) / 2 + c);
    println!("x=2, y={}", (4 * a2 + 2 * b2) / 2 + c);
    println!(
        "x=202300, y={}",
        (202_300 * 202_300 * a2 + 202_300 * b2) / 2 + c
    );
    // 11842978506936380 too high
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

    #[test]
    fn test_expand() {
        let grid = read(INPUT);
        let grid = expand(&grid);
        let grid = steps(grid, 6);
        print_grid(&grid);
        let result = count_locations(&grid);
        assert_eq!(result, 16);
    }
}
