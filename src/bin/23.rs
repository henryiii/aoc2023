/*!
# 2023 Day 23: A Long Walk
## Longest Path

<https://adventofcode.com/2023/day/23>

This is currently a fairly simple brute-force solution using petgraph. I was
able to do part one on my phone in Python, but wasn't willing to wait out the
brute force solution there, but here it's 10 mins or so. Condensing the graph
(see history for old version) made this much, much faster.

Part 2 is just part 1 again but replacing the directional chars with dots.

```python
from pathlib import Path
from contextlib import suppress

grid = Path("23.txt").read_text().splitlines()

start = (0, grid[0].index("."))
end = (len(grid)-1, grid[-1].index("."))

def path(loc, seen):
    while loc != end:
        plan = []
        for d in ((0, 1), (1, 0), (0, -1), (-1, 0)):
            new = loc[0]+d[0], loc[1] + d[1]
            if new in seen:
                continue
            if min(new) >= 0 and new[0] < len(grid) and new[1] < len(grid[0]):
                    next = grid[new[0]][new[1]]
                    match (next, *d):
                        case ('.', _, _) | ('>', 0, 1) | ('<', 0, -1) | ('v', 1, 0) | ('^', -1, 0):
                            plan.append(new)

        match plan:
            case (): break
            case [lo, *o]:
                for x in o:
                    yield from path(x, seen | {x})
                loc = lo
                seen.add(loc)
            case _:
                raise AssertionError(f"invalid {plan}")

    if loc == end:
        yield seen

paths = path(start, {start})
paths = [list(p) for p in paths]
print(sorted((len(p)-1 for p in paths), reverse=True))
```

*/

use grid::Grid;
use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use petgraph::graph::Graph;

fn read_grid(text: &str) -> Grid<char> {
    text.lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
        .into()
}

fn make_graph_directed(grid: &Grid<char>) -> Graph<(usize, usize), usize> {
    let mut graph = Graph::new();
    let nodes = grid
        .indexed_iter()
        .filter_map(|((y, x), c)| {
            if matches!(c, '.' | '<' | '>' | '^' | 'v') {
                Some(graph.add_node((y, x)))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let dirs = [(0, 1), (1, 0)];
    for node in &nodes {
        let (y, x) = graph[*node];
        let current = grid[(y, x)];
        for (dy, dx) in &dirs {
            let (ny, nx) = (
                i32::try_from(y).unwrap() + dy,
                i32::try_from(x).unwrap() + dx,
            );
            if let Some(next) = grid.get(ny, nx) {
                match (current, dy, dx, next) {
                    ('.', _, _, '.')
                    | ('>', 0, 1, _)
                    | (_, 0, 1, '>')
                    | ('v', 1, 0, _)
                    | (_, 1, 0, 'v') => {
                        let other = nodes
                            .iter()
                            .find(|n| {
                                graph[**n]
                                    == (usize::try_from(ny).unwrap(), usize::try_from(nx).unwrap())
                            })
                            .unwrap();
                        graph.add_edge(*node, *other, 1);
                    }
                    _ => {}
                }
                match (current, dy, dx, next) {
                    ('.', _, _, '.')
                    | (_, 0, 1, '<')
                    | ('<', 0, 1, _)
                    | (_, 1, 0, '^')
                    | ('^', 1, 0, _) => {
                        let other = nodes
                            .iter()
                            .find(|n| {
                                graph[**n]
                                    == (usize::try_from(ny).unwrap(), usize::try_from(nx).unwrap())
                            })
                            .unwrap();
                        graph.add_edge(*other, *node, 1);
                    }
                    _ => {}
                }
            }
        }
    }
    graph
}

fn simplify_graph(mut graph: Graph<(usize, usize), usize>) -> Graph<(usize, usize), usize> {
    'main: loop {
        for node in graph.node_indices() {
            if graph.neighbors_undirected(node).unique().count() == 2 {
                let (first, second) = graph
                    .neighbors_undirected(node)
                    .unique()
                    .collect_tuple()
                    .unwrap();
                for (a, b) in [(first, second), (second, first)] {
                    let edges = graph
                        .edges_connecting(a, node)
                        .chain(graph.edges_connecting(node, b));
                    let num_edges = edges.clone().count();
                    let new_weight = edges.map(|x| x.weight()).sum();
                    if num_edges > 1 {
                        graph.add_edge(a, b, new_weight);
                    }
                }
                graph.remove_node(node);
                continue 'main;
            }
        }
        return graph;
    }
}

fn longest_path_length(graph: &Graph<(usize, usize), usize>, grid: &Grid<char>) -> usize {
    let from = graph.node_indices().find(|n| graph[*n].0 == 0).unwrap();
    let to = graph
        .node_indices()
        .find(|n| graph[*n].0 == grid.rows() - 1)
        .unwrap();
    let paths: Vec<_> = all_simple_paths(&graph, from, to, 0, None)
        .map(|x: Vec<_>| {
            x.iter()
                .skip(1)
                .fold((0, &from), |acc, y| {
                    (
                        acc.0
                            + graph
                                .edge_weight(graph.find_edge(*acc.1, *y).unwrap())
                                .unwrap(),
                        y,
                    )
                })
                .0
        })
        .collect();
    *paths.iter().max().unwrap()
}

fn compute1(text: &str) -> usize {
    let grid = read_grid(text);
    let graph = make_graph_directed(&grid);
    let graph = simplify_graph(graph);
    print_graph(&graph);
    longest_path_length(&graph, &grid)
}

fn compute2(text: &str) -> usize {
    let mut grid = read_grid(text);
    for c in grid.iter_mut() {
        if matches!(*c, '<' | '>' | '^' | 'v') {
            *c = '.';
        }
    }
    let graph = make_graph_directed(&grid);
    let graph = simplify_graph(graph);
    print_graph(&graph);
    longest_path_length(&graph, &grid)
}

fn print_graph<T: core::fmt::Debug>(graph: &Graph<T, usize>) {
    use petgraph::dot::Dot;
    print!("{:?}", Dot::with_config(&graph, &[]));
}

fn main() {
    let text = std::fs::read_to_string("input/23.txt").unwrap();
    let result = compute1(&text);
    println!("First = {result}");

    let result = compute2(&text);
    println!("Second = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_first() {
        let result = compute1(INPUT);
        assert_eq!(result, 94);
    }

    #[test]
    fn test_second() {
        let result = compute2(INPUT);
        assert_eq!(result, 154);
    }
}
