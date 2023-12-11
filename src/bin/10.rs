#[derive(Debug, Clone, Copy)]
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

fn pipe_draw(input: &str) -> String {
    input
        .replace('|', "│")
        .replace('L', "└")
        .replace('J', "┘")
        .replace('F', "┌")
        .replace('7', "┐")
        .replace('-', "─")
        .replace('.', " ")
}

#[derive(Debug, Clone)]
struct Cursor<'a> {
    grid: &'a [&'a str],
    x: usize,
    y: usize,
}

impl<'a> Cursor<'a> {
    fn new(grid: &'a [&'a str], x: usize, y: usize) -> Self {
        Self { grid, x, y }
    }

    fn step(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn peek(&self, dir: &Direction) -> Option<char> {
        match dir {
            Direction::Up => self.grid.get(self.y - 1)?.chars().nth(self.x),
            Direction::Down => self.grid.get(self.y + 1)?.chars().nth(self.x),
            Direction::Left => self.grid.get(self.y)?.chars().nth(self.x - 1),
            Direction::Right => self.grid.get(self.y)?.chars().nth(self.x + 1),
        }
    }

    fn find_start(grid: &'a [&'a str]) -> Self {
        let (start_y, startline) = grid
            .iter()
            .enumerate()
            .find(|(_, x)| x.contains('S'))
            .unwrap();
        let start_x = startline.find('S').unwrap();
        Self::new(grid, start_x, start_y)
    }

    fn find_end(&self, start_direction: &Direction) -> usize {
        let mut cursor = self.clone();
        let mut current_dir = *start_direction;
        let mut next_dir: Direction;
        for i in 0.. {
            next_dir = if let Some(next) = cursor.peek(&current_dir) {
                match (next, current_dir) {
                    ('|', Direction::Up) => Direction::Up,
                    ('|', Direction::Down) => Direction::Down,
                    ('-', Direction::Left) => Direction::Left,
                    ('-', Direction::Right) => Direction::Right,
                    ('J', Direction::Right) => Direction::Up,
                    ('J', Direction::Down) => Direction::Left,
                    ('L', Direction::Down) => Direction::Right,
                    ('L', Direction::Left) => Direction::Up,
                    ('7', Direction::Right) => Direction::Down,
                    ('7', Direction::Up) => Direction::Left,
                    ('F', Direction::Left) => Direction::Down,
                    ('F', Direction::Up) => Direction::Right,
                    ('S', _) => return i,
                    _ => panic!("No next direction found"),
                }
            } else {
                panic!("No next direction found");
            };
            cursor.step(&current_dir);
            current_dir = next_dir;
        }
        panic!("No end found");
    }

    fn get_next_from_start(&self) -> &Direction {
        DIRECTIONS
            .iter()
            .find(|dir| {
                if let Some(next) = self.peek(dir) {
                    matches!(
                        (next, dir),
                        ('|', Direction::Up | Direction::Down)
                            | ('-', Direction::Left | Direction::Right)
                            | ('J', Direction::Right | Direction::Down)
                            | ('L', Direction::Left | Direction::Down)
                            | ('7', Direction::Right | Direction::Up)
                            | ('F', Direction::Left | Direction::Up)
                    )
                } else {
                    false
                }
            })
            .unwrap()
    }
}

fn main() {
    let text = std::fs::read_to_string("input/10.txt").unwrap();
    println!("{}", pipe_draw(&text));

    let grid: Vec<&str> = text.lines().collect();
    let cursor = Cursor::find_start(&grid);
    let current_dir = cursor.get_next_from_start();
    let count = cursor.find_end(current_dir);
    println!("Count: {}", (count + 1) / 2);
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test() {
        let grid: Vec<&str> = INPUT2.lines().collect();
        let cursor = Cursor::find_start(&grid);
        let current_dir = cursor.get_next_from_start();
        let count = cursor.find_end(current_dir);
        assert_eq!((count + 1) / 2, 8);
    }
}
