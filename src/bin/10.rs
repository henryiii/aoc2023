#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

static DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MapChar {
    Start,
    Vertical,
    Horizontal,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Empty,
}

impl TryFrom<char> for MapChar {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(Self::Start),
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'J' => Ok(Self::UpLeft),
            'L' => Ok(Self::UpRight),
            '7' => Ok(Self::DownLeft),
            'F' => Ok(Self::DownRight),
            '.' => Ok(Self::Empty),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for MapChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Start => "S",
            Self::Vertical => "│",
            Self::Horizontal => "─",
            Self::UpLeft => "╯",
            Self::UpRight => "╰",
            Self::DownLeft => "╮",
            Self::DownRight => "╭",
            Self::Empty => "•",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone)]
struct Cursor {
    y: usize,
    x: usize,
}

impl Cursor {
    #[must_use]
    const fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    #[must_use]
    const fn step(&self, dir: Direction) -> Self {
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
        assert!(grid[(self.y, self.x)] == MapChar::Start);
        match self.get_from_start(grid) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => {
                &MapChar::Vertical
            }
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => {
                &MapChar::Horizontal
            }
            (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => &MapChar::UpLeft,
            (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => {
                &MapChar::UpRight
            }
            (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => {
                &MapChar::DownLeft
            }
            (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => {
                &MapChar::DownRight
            }
            _ => panic!("Invalid start"),
        }
    }

    #[must_use]
    fn get_from_start(&self, grid: &Grid<MapChar>) -> (Direction, Direction) {
        assert!(grid[(self.y, self.x)] == MapChar::Start);
        let mut valid_dirs = DIRECTIONS.iter().filter(|dir| {
            log::debug!("{self:?} {dir:?}");
            self.peek(grid, **dir).map_or(false, |next| {
                log::debug!("Checking ({next}, {dir:?})");
                matches!(
                    (next, dir),
                    (MapChar::Vertical, Direction::Up | Direction::Down)
                        | (MapChar::Horizontal, Direction::Left | Direction::Right)
                        | (MapChar::UpLeft, Direction::Right | Direction::Down)
                        | (MapChar::UpRight, Direction::Left | Direction::Down)
                        | (MapChar::DownLeft, Direction::Right | Direction::Up)
                        | (MapChar::DownRight, Direction::Left | Direction::Up)
                )
            })
        });
        (*valid_dirs.next().unwrap(), *valid_dirs.next().unwrap())
    }
    #[must_use]
    fn find_end(
        &self,
        start_direction: Direction,
        grid: &Grid<MapChar>,
        mask: &mut Grid<bool>,
    ) -> usize {
        let mut cursor = self.clone();
        let mut current_dir = start_direction;
        let mut next_dir: Direction;
        for i in 0.. {
            mask[(cursor.y, cursor.x)] = true;
            next_dir = if let Some(next) = cursor.peek(grid, current_dir) {
                log::debug!("{i}: {current_dir:?} -> {next}");
                match (next, current_dir) {
                    (MapChar::Vertical, Direction::Up)
                    | (MapChar::UpLeft, Direction::Right)
                    | (MapChar::UpRight, Direction::Left) => Direction::Up,
                    (MapChar::Vertical, Direction::Down)
                    | (MapChar::DownLeft, Direction::Right)
                    | (MapChar::DownRight, Direction::Left) => Direction::Down,
                    (MapChar::Horizontal, Direction::Left)
                    | (MapChar::UpLeft, Direction::Down)
                    | (MapChar::DownLeft, Direction::Up) => Direction::Left,
                    (MapChar::Horizontal, Direction::Right)
                    | (MapChar::UpRight, Direction::Down)
                    | (MapChar::DownRight, Direction::Up) => Direction::Right,
                    (MapChar::Start, _) => return i,
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
    let (y, min_x) = *loc;
    let mut crossings = 0;
    let mut enter = None;
    for x in (min_x)..(grid.cols()) {
        if mask[(y, x)] {
            let mut c = grid[(y, x)];
            if c == MapChar::Start {
                c = *Cursor::new(y, x).compute_start_char(grid);
            }
            match (c, enter) {
                (MapChar::Vertical, _) => crossings += 1,
                (MapChar::DownRight, None) => enter = Some(Direction::Down),
                (MapChar::UpRight, None) => enter = Some(Direction::Up),
                (MapChar::DownLeft, Some(Direction::Down))
                | (MapChar::UpLeft, Some(Direction::Up)) => enter = None,
                (MapChar::DownLeft, Some(Direction::Up))
                | (MapChar::UpLeft, Some(Direction::Down)) => {
                    enter = None;
                    crossings += 1;
                }
                (MapChar::Horizontal, Some(_)) => (),
                _ => panic!("Invalid char {c:?} at ({y}, {x}), enter={enter:?}"),
            }
        }
    }
    crossings % 2 == 1
}

fn compute_and_print_grid(strs: &[&str]) -> (usize, usize) {
    let grid = Grid::from_vec(
        strs.iter()
            .flat_map(|x| x.chars().map(|x| MapChar::try_from(x).unwrap()))
            .collect(),
        strs[0].len(),
    );
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
