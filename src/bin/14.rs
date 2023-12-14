use grid::Grid;
use std::str::FromStr;

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
    grid.indexed_iter().filter_map(|((y, x), c)| {
        if *c == Map::RoundRock {
            Some(grid.rows() - y)
        } else {
            None
        }
    }).sum()
}

fn tilt_north(grid: &mut Grid<Map>) {
    for x in 0..grid.cols() {
        for y in 0..grid.rows() {
            if grid[(y, x)] == Map::RoundRock {
                for i in 0..y {
                    if grid[(y-i-1, x)] == Map::Empty {
                        grid[(y-i-1, x)] = Map::RoundRock;
                        grid[(y-i, x)] = Map::Empty;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn compute(text: &str) -> Num {
    let mut grid = read_data(text);
    print_grid(&grid);
    println!();
    tilt_north(&mut grid);
    print_grid(&grid);
    compute_load(&grid)
}

fn main() {
    let text = std::fs::read_to_string("input/14.txt").unwrap();
    let first_result = compute(&text);
    println!("First = {first_result}");

    let second_result = compute(&text);
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
        let result = compute(INPUT);
        assert_eq!(result, 136);
    }
}
