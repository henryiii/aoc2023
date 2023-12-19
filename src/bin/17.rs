/*!
# 2023 Day 17 - Minimizing the path

<https://adventofcode.com/2023/day/17>


Credit to <https://www.youtube.com/watch?v=2pDSooPLLkI> for helping point me
toward Dijkstra's algorithm. And
<https://doc.rust-lang.org/std/collections/binary_heap/index.html> for a great
example of the algorithm to base it on..
*/

use std::collections::BinaryHeap;
use std::{cmp::Ordering, collections::HashSet};

use grid::Grid;

use aoc2023::grid_helper::{Direction, Position};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position,
    direction: Direction,
    len: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min cost. Other values needed for stability.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.len.cmp(&other.len))
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_grid(text: &str) -> Grid<usize> {
    let vals: Vec<usize> = text
        .lines()
        .flat_map(|x| x.chars().map(|x| x.to_string().parse().unwrap()))
        .collect();

    Grid::from_vec(vals, text.lines().next().unwrap().len())
}

fn compute_path(grid: &Grid<usize>, min_path: usize, max_path: usize) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut computed = HashSet::new();
    let mut dist = Grid::new(grid.rows(), grid.cols());
    let goal = Position::new(
        isize::try_from(grid.rows()).unwrap() - 1,
        isize::try_from(grid.cols()).unwrap() - 1,
    );

    heap.push(State {
        cost: 0,
        position: Position::new(0, 0),
        direction: Direction::Right,
        len: 0,
    });

    while let Some(State {
        cost,
        position,
        direction,
        len,
    }) = heap.pop()
    {
        if position == goal {
            return Some(cost);
        }
        if !computed.insert((position, direction, len)) {
            continue;
        }
        let directions = vec![
            direction,
            direction.clockwise(),
            direction.counter_clockwise(),
        ];
        'dir: for dir in directions {
            let new_len = if dir == direction && len != 0 {
                len + 1
            } else {
                min_path
            };
            let steps = if dir == direction && len != 0 {
                1
            } else {
                min_path
            };
            let mut next = position;
            let mut new_cost = cost;
            for _ in 0..steps {
                next = next + dir;
                if let Some(val) = grid.get(next.row(), next.col()) {
                    new_cost += val;
                } else {
                    continue 'dir;
                }
            }
            if new_len <= max_path {
                let prev_cost = *dist.get(next.row(), next.col()).unwrap();
                if prev_cost == 0 || new_cost < prev_cost + 9 * min_path {
                    heap.push(State {
                        cost: new_cost,
                        position: next,
                        direction: dir,
                        len: new_len,
                    });
                }
                if prev_cost == 0 || new_cost < prev_cost {
                    dist[next] = new_cost;
                }
            }
        }
    }
    None
}

fn compute1(text: &str) -> usize {
    let grid = read_grid(text);
    compute_path(&grid, 1, 3).unwrap()
}

fn compute2(text: &str) -> usize {
    let grid = read_grid(text);
    compute_path(&grid, 4, 10).unwrap()
}

// Too low: 1147
// Too high: 1190

fn main() {
    let text = std::fs::read_to_string("input/17.txt").unwrap();
    let result = compute1(&text);
    println!("First = {result}");

    let result = compute2(&text);
    println!("Second = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const INPUT2: &str = "\
111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test_first() {
        let result = compute1(INPUT);
        assert_eq!(result, 102);
    }

    #[test]
    fn test_second() {
        let result = compute2(INPUT);
        assert_eq!(result, 94);
    }

    #[test]
    fn test_second_2() {
        let result = compute2(INPUT2);
        assert_eq!(result, 71);
    }
}
