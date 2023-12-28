/*!
# 2023 Day 25 - Snowverload
## Cutting connections

<https://adventofcode.com/2023/day/25>

This is another one that was trivial in Python (due to the `networkx` library),
and so applying the same solution in Rust using `rustworkx-core`.

```python
from pathlib import Path
import networkx as nx

def read(fn):
    txt = Path(fn).read_text()
    lines = [t.replace(":", "").split() for t in txt.splitlines()]
    return {a[0]: frozenset(a[1:]) for a in lines}

data = read("25data.txt")

graph = nx.from_dict_of_lists(data)
cuts = nx.minimum_edge_cut(graph)
graph.remove_edges(cuts)

a,b = (graph.subgraph(c) for c in nx.connected_components(graph))
print(len(a), "*", len(b), "=", len(a) * len(b))
```
*/

use petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

fn read(text: &str) -> UnGraph<&str, ()> {
    let mut graph = UnGraph::new_undirected();
    let mut nodes = std::collections::HashMap::new();
    for line in text.lines() {
        let (node, edges) = line.split_once(": ").unwrap();
        let node = *nodes.entry(node).or_insert_with(|| graph.add_node(node));
        for edge in edges.split(' ') {
            let edge = *nodes.entry(edge).or_insert_with(|| graph.add_node(edge));
            graph.add_edge(node, edge, ());
        }
    }
    graph
}

fn compute(text: &str) -> usize {
    let graph = read(text);
    let len = find_edges(&graph);
    len * (graph.node_count() - len)
}

fn find_edges(graph: &UnGraph<&str, ()>) -> usize {
    let (cut, items) = stoer_wagner_min_cut(graph, |_| Ok::<_, ()>(1))
        .unwrap()
        .unwrap();
    assert_eq!(cut, 3);
    items.len()
}

fn main() {
    let text = std::fs::read_to_string("input/25.txt").unwrap();
    let result = compute(&text);
    println!("Answer = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_first() {
        let result = compute(INPUT);
        assert_eq!(result, 54);
    }
}
