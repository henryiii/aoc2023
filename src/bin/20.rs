/*!
# 2023 Day 20 - Binary node network

<https://adventofcode.com/2023/day/20>


First part is pretty easy, but second part requires a lot of thinking.  Got to
use `RefCell` to make a portion of a struct mutable for the first time here.
*/

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
enum Module<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Pulse>),
    Broadcaster,
    Output((usize, usize)),
}

impl<'a> Module<'a> {
    fn process_pulse(&mut self, pulse: Pulse, sender: &str) -> Option<Pulse> {
        match *self {
            Module::FlipFlop(ref mut flipflop) => {
                if pulse == Pulse::Low {
                    *flipflop = !*flipflop;
                    Some(if *flipflop { Pulse::High } else { Pulse::Low })
                } else {
                    None
                }
            }
            Module::Conjunction(ref mut conj) => {
                assert!(conj.contains_key(sender), "{sender} not in {conj:?}");
                *conj.get_mut(sender).unwrap() = pulse;
                Some(if conj.values().all(|x| *x == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                })
            }
            Module::Broadcaster => Some(pulse),
            Module::Output(ref mut output) => {
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
            Module::FlipFlop(ref flipflop) => !flipflop,
            Module::Conjunction(ref conj) => conj.values().all(|x| *x == Pulse::Low),
            _ => true,
        }
    }
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    module: RefCell<Module<'a>>,
    output: Vec<&'a str>,
}

impl<'a> Node<'a> {
    fn broadcast(&self, sender: &str, pulse: Pulse) -> (Vec<&str>, Vec<&str>) {
        let send = self.module.borrow_mut().process_pulse(pulse, sender);
        log::debug!(
            "{sender} -{pulse:?}- -> {} (sending {send:?} to output {:?})",
            self.name,
            self.output
        );
        match send {
            None => (Vec::new(), Vec::new()),
            Some(Pulse::High) => (self.output.clone(), Vec::new()),
            Some(Pulse::Low) => (Vec::new(), self.output.clone()),
        }
    }
}

fn read_input(text: &str) -> HashMap<&str, Node> {
    let mut node_map: HashMap<&str, Node> = text
        .lines()
        .map(|line| {
            let (inp, output) = line.split_once(" -> ").unwrap();
            let output = output.split(", ").collect();
            if let Some(name) = inp.strip_prefix('%') {
                Node {
                    name,
                    module: RefCell::new(Module::FlipFlop(false)),
                    output,
                }
            } else if let Some(name) = inp.strip_prefix('&') {
                Node {
                    name,
                    module: RefCell::new(Module::Conjunction(HashMap::new())),
                    output,
                }
            } else if inp == "broadcaster" {
                Node {
                    name: inp,
                    module: RefCell::new(Module::Broadcaster),
                    output,
                }
            } else {
                panic!("Unknown input: {inp}");
            }
        })
        .map(|n| (n.name, n))
        .collect();
    let name_output: Vec<(_, _)> = node_map
        .iter()
        .flat_map(|(name, node)| node.output.iter().map(move |x| (*name, *x)))
        .collect();
    for (name, outp) in name_output {
        if let Some(node) = node_map.get(outp) {
            if let Module::Conjunction(ref mut conj) = *node.module.borrow_mut() {
                conj.insert(name, Pulse::Low);
            }
        }
    }
    let all_output_nodes: HashSet<&str> = node_map
        .values()
        .flat_map(|n| n.output.iter().copied())
        .collect();
    let all_input_nodes: HashSet<&str> = node_map.keys().copied().collect();
    let output_only_nodes = all_output_nodes.difference(&all_input_nodes);
    for name in output_only_nodes {
        node_map.insert(
            name,
            Node {
                name,
                module: RefCell::new(Module::Output((0, 0))),
                output: Vec::new(),
            },
        );
    }
    node_map
}

fn compute_press(node_map: &HashMap<&str, Node>) -> (u64, u64) {
    let mut high_count = 0;
    let mut low_count = 1;
    let mut high_pulses = Vec::new();
    let mut low_pulses = vec![("button", "broadcaster")];
    while !high_pulses.is_empty() || !low_pulses.is_empty() {
        let mut high_tmp = Vec::new();
        let mut low_tmp = Vec::new();
        for (sender, name) in high_pulses {
            let node = &node_map[name];
            let (high_out, low_out) = node.broadcast(sender, Pulse::High);
            high_tmp.extend(high_out.iter().map(|x| (name, *x)));
            low_tmp.extend(low_out.iter().map(|x| (name, *x)));
        }
        for (sender, name) in low_pulses {
            let node = &node_map[name];
            let (high_out, low_out) = node.broadcast(sender, Pulse::Low);
            high_tmp.extend(high_out.iter().map(|x| (name, *x)));
            low_tmp.extend(low_out.iter().map(|x| (name, *x)));
        }
        high_count += high_tmp.len() as u64;
        low_count += low_tmp.len() as u64;
        high_pulses = high_tmp;
        low_pulses = low_tmp;
    }
    (high_count, low_count)
}

fn measure_cycle(node_map: &HashMap<&str, Node>, len: usize) -> (Option<u64>, u64, u64) {
    let mut high_count = 0;
    let mut low_count = 0;
    let mut cycles = 0;
    for _ in 0..len {
        let (high, low) = compute_press(node_map);
        log::debug!("Loop {cycles}");
        high_count += high;
        low_count += low;
        cycles += 1;
        if node_map.values().all(|n| n.module.borrow().is_reset()) {
            return (Some(cycles), high_count, low_count);
        }
    }
    (None, high_count, low_count)
}

fn compute1(text: &str) -> u64 {
    let node_map = read_input(text);
    let (cycles, high, low) = measure_cycle(&node_map, 1000);
    println!("cycles: {cycles:?}, high: {high}, low: {low}");
    cycles.map_or(high * low, |cycles| {
        high * 1000 * low * 1000 / (cycles * cycles)
    })
}

fn print_node_map(node_map: &HashMap<&str, Node>) {
    println!("  stateDiagram-v2");
    println!("    classDef conj fill:#f66");
    println!("    [*] --> broadcaster");
    for (name, node) in node_map {
        for output in &node.output {
            if let Module::Conjunction(_) = &*node.module.borrow() {
                println!("    {name}:::conj --> {output}");
            } else {
                println!("    {name} --> {output}");
            }
        }
    }
    println!("    rx --> [*]");
    println!();
}

fn compute2(text: &str) -> u64 {
    let node_map = read_input(text);
    println!("Initial state:");
    print_node_map(&node_map);
    println!("Assuming certain structure, we can simplify");
    let mut total = Vec::new();
    for node_name in &node_map["broadcaster"].output {
        let mut node = &node_map[node_name];
        let mut number = 0;
        for i in 0.. {
            match node
                .output
                .iter()
                .filter(|x| matches!(*node_map[**x].module.borrow(), Module::FlipFlop(_)))
                .count()
            {
                0 => {
                    number += 1 << i;
                    break;
                }
                1 => {
                    if node.output.len() > 1 {
                        number += 1 << i;
                    }
                    node = &node_map[node
                        .output
                        .iter()
                        .find(|x| matches!(*node_map[*x].module.borrow(), Module::FlipFlop(_)))
                        .unwrap()];
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
        assert_eq!(result, 32000000);
    }
    #[test]
    fn test_second() {
        let result = compute1(INPUT2);
        assert_eq!(result, 11687500);
    }
}
