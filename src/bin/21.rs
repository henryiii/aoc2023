/*!
# 2023 Day 21 - Garden maps

<https://adventofcode.com/2023/day/21>

*/

use grid::Grid;

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

fn compute1(text: &str, dist: i32) -> usize {
    let mut grid = read(text);
    for i in 0..dist {
        let mut new_grid = grid.clone();
        for y in 0..grid.rows() {
            for x in 0..grid.cols() {
                if grid[(y, x)] == i {
                    for (dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        if let Some(v) = grid.get(
                            i32::try_from(y).unwrap() + dy,
                            i32::try_from(x).unwrap() + dx,
                        ) {
                            if *v >= -1 {
                                *new_grid
                                    .get_mut(
                                        i32::try_from(y).unwrap() + dy,
                                        i32::try_from(x).unwrap() + dx,
                                    )
                                    .unwrap() = i + 1;
                                new_grid[(y, x)] = -1;
                            }
                        }
                    }
                }
            }
        }
        grid = new_grid;
    }
    println!("After {dist} steps:");
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
    grid.iter().filter(|x| **x > 0).count()
}

fn main() {
    let text = std::fs::read_to_string("input/21.txt").unwrap();
    let result = compute1(&text, 64);
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
        let result = compute1(INPUT, 6);
        assert_eq!(result, 16);
    }
}
