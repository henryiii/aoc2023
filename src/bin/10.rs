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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum MapChar {
    Start,
    Vertical,
    Horizantal,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Empty,
}

impl From<char> for MapChar {
    fn from(c: char) -> Self {
        match c {
            'S' => MapChar::Start,
            '|' => MapChar::Vertical,
            '-' => MapChar::Horizantal,
            'J' => MapChar::UpLeft,
            'L' => MapChar::UpRight,
            '7' => MapChar::DownLeft,
            'F' => MapChar::DownRight,
            '.' => MapChar::Empty,
            _ => panic!("Invalid char"),
        }
    }
}

impl std::fmt::Display for MapChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MapChar::Start => "S",
            MapChar::Vertical => "│",
            MapChar::Horizantal => "─",
            MapChar::UpLeft => "╯",
            MapChar::UpRight => "╰",
            MapChar::DownLeft => "╮",
            MapChar::DownRight => "╭",
            MapChar::Empty => " ",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
struct Cursor {
    y: usize,
    x: usize,
}

impl Cursor {
    #[must_use]
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    #[must_use]
    fn step(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => Self::new(self.y - 1, self.x),
            Direction::Down => Self::new(self.y + 1, self.x),
            Direction::Left => Self::new(self.y, self.x - 1),
            Direction::Right => Self::new(self.y, self.x + 1),
        }
    }

    #[must_use]
    fn peek<'a, T>(&self, grid: &'a Grid<T>, dir: &Direction) -> Option<&'a T> {
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
    fn find_end(
        &self,
        start_direction: &Direction,
        grid: &Grid<MapChar>,
        mask: &mut Grid<bool>,
    ) -> usize {
        let mut cursor = self.clone();
        let mut current_dir = *start_direction;
        let mut next_dir: Direction;
        for i in 0.. {
            mask[(cursor.y, cursor.x)] = true;
            next_dir = if let Some(next) = cursor.peek(grid, &current_dir) {
                log::debug!("{i}: {current_dir:?} -> {next}");
                match (next, current_dir) {
                    (MapChar::Vertical, Direction::Up) => Direction::Up,
                    (MapChar::Vertical, Direction::Down) => Direction::Down,
                    (MapChar::Horizantal, Direction::Left) => Direction::Left,
                    (MapChar::Horizantal, Direction::Right) => Direction::Right,
                    (MapChar::UpLeft, Direction::Right) => Direction::Up,
                    (MapChar::UpLeft, Direction::Down) => Direction::Left,
                    (MapChar::UpRight, Direction::Down) => Direction::Right,
                    (MapChar::UpRight, Direction::Left) => Direction::Up,
                    (MapChar::DownLeft, Direction::Right) => Direction::Down,
                    (MapChar::DownLeft, Direction::Up) => Direction::Left,
                    (MapChar::DownRight, Direction::Left) => Direction::Down,
                    (MapChar::DownRight, Direction::Up) => Direction::Right,
                    (MapChar::Start, _) => return i,
                    _ => panic!("No next direction found"),
                }
            } else {
                panic!("No next direction found");
            };
            cursor = cursor.step(&current_dir);
            current_dir = next_dir;
        }
        panic!("No end found");
    }

    #[must_use]
    fn get_next_from_start(&self, grid: &Grid<MapChar>) -> &Direction {
        DIRECTIONS
            .iter()
            .find(|dir| {
                log::debug!("{self:?} {dir:?}");
                if let Some(next) = self.peek(grid, dir) {
                    log::debug!("Checking ({next}, {dir:?})");
                    matches!(
                        (next, dir),
                        (MapChar::Vertical, Direction::Up | Direction::Down)
                            | (MapChar::Horizantal, Direction::Left | Direction::Right)
                            | (MapChar::UpLeft, Direction::Right | Direction::Down)
                            | (MapChar::UpRight, Direction::Left | Direction::Down)
                            | (MapChar::DownLeft, Direction::Right | Direction::Up)
                            | (MapChar::DownRight, Direction::Left | Direction::Up)
                    )
                } else {
                    false
                }
            })
            .unwrap()
    }
}

fn compute_and_print_grid(strs: &[&str]) -> (usize, usize) {
    let grid = Grid::from_vec(
        strs.iter()
            .flat_map(|x| x.chars().map(MapChar::from))
            .collect(),
        strs[0].len(),
    );
    let mut mask: Grid<bool> = Grid::new(grid.rows(), grid.cols());

    let cursor = Cursor::find_start(&grid);
    let current_dir = cursor.get_next_from_start(&grid);
    let count = cursor.find_end(current_dir, &grid, &mut mask);
    for (y, row) in grid.iter_rows().enumerate() {
        for (x, c) in row.enumerate() {
            let s = c.to_string();
            if mask[(y, x)] {
                print!("\x1b[93m{s}\x1b[0m");
            } else {
                print!("{s}");
            }
        }
        println!();
    }

    let internal: usize = 0;
    (((count + 1) / 2), internal)
}

fn main() {
    env_logger::init();
    let text = std::fs::read_to_string("input/10.txt").unwrap();
    let grid: Vec<&str> = text.lines().collect();
    let (count, internal) = compute_and_print_grid(&grid);

    println!("Count: {}", count);
    println!("Internal: {}", internal);
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
