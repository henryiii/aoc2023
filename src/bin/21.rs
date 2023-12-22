/*!
# 2023 Day 21 - Garden maps

<https://adventofcode.com/2023/day/21>

Part 1 was pretty easy, just avoiding mistakes was tricky. I basically just
do a convolution map [0 1 0; 1 0 1; 0 1 0] by hand. Part 2 was tricky, and
had to look around for a bit of insperation. I basically just expand and
compute the first couple of board-lenths (131), and then solve; the puzzle
boards are all clear around the diagonals and edges, so each expansion adds
the same amount (partially squared) to the total.

This was rewritten to aboid copying the mask and to avoid grid copies too.  It
is much, much faster than the original version, and supports arbitray numbers of
steps.
*/

use grid::Grid;

/// This could be a inline function with `impl TryFrom<i32>`, but making it
/// a macro allows us to reuse the name, which is fun. This also puts the
/// error message in the right spot.
#[macro_export]
macro_rules! i32 {
    ($x:expr) => {
        i32::try_from($x).unwrap()
    };
}

#[macro_export]
macro_rules! usize {
    ($x:expr) => {
        usize::try_from($x).unwrap()
    };
}

#[must_use]
fn read(text: &str) -> (Grid<bool>, (usize, usize)) {
    let mask: Grid<bool> = text
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '#' => false,
                    '.' | 'S' => true,
                    _ => panic!("Unknown char {x}"),
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .into();

    let (y, x) = text
        .lines()
        .enumerate()
        .find_map(|(y, line)| Some((y, line.find('S')?)))
        .unwrap();

    assert_eq!(y, mask.rows() / 2);
    assert_eq!(x, mask.cols() / 2);
    (mask, (y, x))
}

#[must_use]
fn steps(mask: &Grid<bool>, start: &(usize, usize), dist: usize) -> Grid<bool> {
    let mut grid: Grid<bool> = Grid::new(2 * dist + 1, 2 * dist + 1);
    let midpoint = (i32!(grid.rows() - 1) / 2, i32!(grid.cols() - 1) / 2);
    let start = (i32!(start.0), i32!(start.1));
    grid[(usize!(midpoint.0), usize!(midpoint.1))] = true;
    for i in 0..i32!(dist) {
        // y, x are distances from the center, in grid coords
        for y in -i..=i {
            for x in -i..=i {
                // In order to avoid copies, we can use a checkerboard pattern
                if i % 2 != (x + y).rem_euclid(2) {
                    continue;
                }
                // yc, xc are distance from the corner, grid coords
                let yc = y + midpoint.0;
                let xc = x + midpoint.1;
                // If this value is true, we swap with four surrounding values
                if *grid.get(yc, xc).unwrap() {
                    for (dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        // ty, tx are the new values, in grid coords
                        let ty = yc + dy;
                        let tx = xc + dx;

                        // my, mx is the new values, in mask coords
                        let my = usize!((ty + start.0 - midpoint.0).rem_euclid(i32!(mask.rows())));
                        let mx = usize!((tx + start.1 - midpoint.1).rem_euclid(i32!(mask.cols())));

                        if mask[(my, mx)] {
                            grid[(usize!(ty), usize!(tx))] = true;
                            grid[(usize!(yc), usize!(xc))] = false;
                        }
                    }
                }
            }
        }
    }
    grid
}

#[must_use]
fn count_locations(grid: &Grid<bool>) -> usize {
    grid.iter().filter(|x| **x).count()
}

fn print_grid(mask: &Grid<bool>, grid: &Grid<bool>, start: &(usize, usize)) {
    let midpoint = ((grid.rows() - 1) / 2, (grid.cols() - 1) / 2);
    for yc in 0..grid.rows() {
        for xc in 0..grid.cols() {
            let y = (i32!(yc + start.0) - i32!(midpoint.0)).rem_euclid(i32!(mask.rows()));
            let x = (i32!(xc + start.1) - i32!(midpoint.1)).rem_euclid(i32!(mask.cols()));
            let c = match (mask[(usize!(y), usize!(x))], grid[(yc, xc)]) {
                (false, false) => '#',
                (true, false) => '.',
                (true, true) => 'O',
                (false, true) => panic!("Can't exist in both at {y}, {x}!"),
            };
            if yc == grid.rows() / 2 && xc == grid.cols() / 2 {
                print!("S");
            } else {
                print!("{c}");
            }
        }
        println!();
    }
}

#[must_use]
fn compute1(text: &str) -> usize {
    let (mask, start) = read(text);
    let grid = steps(&mask, &start, 64);
    print_grid(&mask, &grid, &start);
    count_locations(&grid)
}

fn main() {
    let text = std::fs::read_to_string("input/21.txt").unwrap();
    let result = compute1(&text);
    println!("First = {result}");

    let val = 26_501_365;
    println!("Going to compute for steps = {val}");

    let (mask, start) = read(&text);
    let grid = steps(&mask, &start, 65);
    let y_0 = count_locations(&grid);
    println!("131*0 = 65 = {y_0}");

    let grid = steps(&mask, &start, 65 + 131);
    let y_1 = count_locations(&grid);
    println!("131*1 + 65 = {y_1}");

    let grid = steps(&mask, &start, 65 + 131 * 2);
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
        let (mask, start) = read(INPUT);
        let grid = steps(&mask, &start, 6);
        let result = count_locations(&grid);
        print_grid(&mask, &grid, &start);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_second() {
        let (mask, start) = read(INPUT);
        let grid = steps(&mask, &start, 10);
        let result = count_locations(&grid);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_third() {
        let (mask, start) = read(INPUT);
        let grid = steps(&mask, &start, 50);
        let result = count_locations(&grid);
        assert_eq!(result, 1594);
    }
}
