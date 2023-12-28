/*!
# 2023 Day 20: Pulse Propagation
## Binary node network

<https://adventofcode.com/2023/day/20>


First part is pretty easy, but second part requires a lot of thinking.  Got to
use `RefCell` to make a portion of a struct mutable for the first time here
(history).  I reworked the implementation to use a graph, which is a lot cleaner
(and sadely avoids the `RefCell`). This is a bit more complex than it it needs
to be (`Node` could be removed) but it allows a Dot graph (see history for
hand-implemented mermaid graph).
*/

use core::fmt::{Debug, Formatter};
use std::collections::{HashMap, HashSet};

use derive_more::Constructor;
use itertools::Itertools;
use petgraph::{
    graph::Graph,
    graph::NodeIndex,
    Direction::{Incoming, Outgoing},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<NodeIndex<u32>, Pulse>),
    Broadcaster,
    Output((usize, usize)),
}

#[derive(Constructor)]
struct Node {
    name: String,
    module: Module,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        match &self.module {
            Module::FlipFlop(val) => write!(f, "{name}: %({val})"),
            Module::Conjunction(map) => write!(f, "{name}: &({})", map.len()),
            Module::Broadcaster => write!(f, "{name}"),
            Module::Output((high, low)) => write!(f, "{name} -> ({high}, {low})"),
        }
    }
}

type ModuleGraph = Graph<Node, ()>;

impl Module {
    fn process_pulse(&mut self, pulse: Pulse, sender: NodeIndex) -> Option<Pulse> {
        match *self {
            Self::FlipFlop(ref mut flipflop) => {
                if pulse == Pulse::Low {
                    *flipflop = !*flipflop;
                    Some(if *flipflop { Pulse::High } else { Pulse::Low })
                } else {
                    None
                }
            }
            Self::Conjunction(ref mut conj) => {
                *conj.get_mut(&sender).unwrap() = pulse;
                Some(if conj.values().all(|x| *x == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                })
            }
            Self::Broadcaster => Some(pulse),
            Self::Output(ref mut output) => {
                if pulse == Pulse::High {
                    output.0 += 1;
                } else {
                    output.1 += 1;
                }
                None
            }
        }
    }

    fn is_reset(&self) -> bool {
        match *self {
            Self::FlipFlop(ref flipflop) => !flipflop,
            Self::Conjunction(ref conj) => conj.values().all(|x| *x == Pulse::Low),
            _ => true,
        }
    }
}

fn read_input(text: &str) -> ModuleGraph {
    let mut graph = ModuleGraph::new();
    let info: Vec<(&str, Vec<&str>)> = text
        .lines()
        .map(|line| {
            let (inp, output) = line.split_once(" -> ").unwrap();
            let output = output.split(", ").collect();
            (inp, output)
        })
        .collect();

    let all_output_nodes: HashSet<&str> = info.iter().flat_map(|(_, n)| n.clone()).collect();
    let flip_flop_nodes: HashSet<&str> = info
        .iter()
        .filter_map(|(n, _)| n.strip_prefix('%'))
        .collect();
    let conjunction_nodes: HashSet<&str> = info
        .iter()
        .filter_map(|(n, _)| n.strip_prefix('&'))
        .collect();
    let in_nodes = HashSet::from([info.iter().find(|(x, _)| *x == "broadcaster").unwrap().0]);
    let all_input_nodes = &(&flip_flop_nodes | &conjunction_nodes) | &in_nodes;
    let output_only_nodes = &all_output_nodes - &all_input_nodes;

    // Temporary map of strings to indexes
    let mut node_map = HashMap::new();

    for node in flip_flop_nodes {
        let inode = graph.add_node(Node::new(node.to_string(), Module::FlipFlop(false)));
        node_map.insert(node, inode);
    }
    for node in conjunction_nodes {
        let inode = graph.add_node(Node::new(
            node.to_string(),
            Module::Conjunction(HashMap::new()),
        ));
        node_map.insert(node, inode);
    }
    for node in in_nodes {
        let inode = graph.add_node(Node::new(node.to_string(), Module::Broadcaster));
        node_map.insert(node, inode);
    }
    for node in output_only_nodes {
        let inode = graph.add_node(Node::new(node.to_string(), Module::Output((0, 0))));
        node_map.insert(node, inode);
    }

    // Add edges
    for (inp, output) in info {
        let inp = inp
            .strip_prefix('%')
            .unwrap_or_else(|| inp.strip_prefix('&').unwrap_or(inp));
        let in_node = node_map[inp];
        for outp in output {
            let out_node = node_map[outp];
            graph.add_edge(in_node, out_node, ());
        }
    }

    // Set up Conj nodes
    for node in graph.node_indices() {
        let module = &graph[node].module;
        let mut edges: Vec<NodeIndex> = Vec::new();
        if let Module::Conjunction { .. } = module {
            edges = graph.neighbors_directed(node, Incoming).collect();
        }
        if !edges.is_empty() {
            let module = &mut graph[node].module;
            *module = Module::Conjunction(edges.iter().map(|k| (*k, Pulse::Low)).collect());
        }
    }

    graph
}

fn compute_press(node_graph: &mut ModuleGraph) -> (u64, u64) {
    type PulseTuple = (Pulse, NodeIndex, NodeIndex);

    let (broadcast,) = node_graph.externals(Incoming).collect_tuple().unwrap();
    let mut high_count = 0;
    let mut low_count = 1;
    let mut pulses: Vec<PulseTuple> = vec![(Pulse::Low, broadcast, broadcast)];
    while !pulses.is_empty() {
        let tmp_pulses: Vec<PulseTuple> = pulses
            .iter()
            .flat_map(|(pulse, sender, current)| {
                let node = &mut node_graph[*current].module;
                let send = node.process_pulse(*pulse, *sender);
                send.map_or_else(Vec::new, |new_pulse| {
                    node_graph
                        .neighbors_directed(*current, Outgoing)
                        .map(|x| (new_pulse, *current, x))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        let (high_tmp, low_tmp): (Vec<PulseTuple>, Vec<PulseTuple>) = tmp_pulses
            .iter()
            .partition(|(pulse, _, _)| *pulse == Pulse::High);
        high_count += high_tmp.len() as u64;
        low_count += low_tmp.len() as u64;
        pulses = tmp_pulses;
    }
    (high_count, low_count)
}

fn measure_cycle(node_graph: &mut ModuleGraph, len: usize) -> (Option<u64>, u64, u64) {
    let mut high_count = 0;
    let mut low_count = 0;
    let mut cycles = 0;
    for _ in 0..len {
        let (high, low) = compute_press(node_graph);
        log::debug!("Loop {cycles}");
        high_count += high;
        low_count += low;
        cycles += 1;
        if node_graph.node_weights().all(|x| x.module.is_reset()) {
            return (Some(cycles), high_count, low_count);
        }
    }
    (None, high_count, low_count)
}

fn compute1(text: &str) -> u64 {
    let mut node_map = read_input(text);
    let (cycles, high, low) = measure_cycle(&mut node_map, 1000);
    println!("cycles: {cycles:?}, high: {high}, low: {low}");
    cycles.map_or(high * low, |cycles| {
        high * 1000 * low * 1000 / (cycles * cycles)
    })
}

fn print_node_map(node_graph: &ModuleGraph) {
    use petgraph::dot::{Config, Dot};
    println!(
        "{:?}",
        Dot::with_config(&node_graph, &[Config::EdgeNoLabel])
    );
}

fn compute2(text: &str) -> u64 {
    let node_graph = read_input(text);
    println!("Assuming certain structure, we can simplify");

    let (broadcast,) = node_graph.externals(Incoming).collect_tuple().unwrap();
    let mut total = Vec::new();

    for node_idx in node_graph.neighbors_directed(broadcast, Outgoing) {
        let mut node = node_idx;
        let mut number = 0;
        for i in 0.. {
            let children: Vec<NodeIndex> = node_graph.neighbors_directed(node, Outgoing).collect();
            let flip_flops: Vec<NodeIndex> = children
                .iter()
                .filter(|x| matches!(node_graph[**x].module, Module::FlipFlop { .. }))
                .copied()
                .collect();
            match flip_flops.len() {
                0 => {
                    number += 1 << i;
                    break;
                }
                1 => {
                    if children.len() > 1 {
                        number += 1 << i;
                    }
                    node = flip_flops[0];
                }
                _ => unreachable!(),
            }
        }
        total.push(number);
    }
    for n in &total {
        print!("{n:b} ");
    }
    println!();
    println!("Part 2: {total:?}");
    total.iter().product()
}

fn main() {
    env_logger::init();
    let text = std::fs::read_to_string("input/20.txt").unwrap();
    let node_map = read_input(&text);
    println!("DOT graph:");
    println!();
    print_node_map(&node_map);
    println!();

    let result = compute1(&text);
    println!("First = {result}");

    let result = compute2(&text);
    println!("Second = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const INPUT: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    const INPUT2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_first() {
        let result = compute1(INPUT);
        assert_eq!(result, 32_000_000);
    }

    #[test]
    fn test_second() {
        let result = compute1(INPUT2);
        assert_eq!(result, 11_687_500);
    }
}
