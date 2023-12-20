/*!
# 2023 Day 20 - ...

<https://adventofcode.com/2023/day/20>

*/

use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct FlipFlop {
    state: bool,
}

#[derive(Debug)]
struct Conjunction<'a> {
    inputs: HashMap<&'a str, Pulse>,
}

#[derive(Debug)]
enum Module<'a> {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction<'a>),
    Broadcaster,
    Output,
}

impl<'a> Module<'a> {
    fn process_pulse(&mut self, pulse: Pulse, sender: &str) -> Option<Pulse> {
        match *self {
            Module::FlipFlop(ref mut flipflop) => {
                if pulse == Pulse::Low {
                    flipflop.state = !flipflop.state;
                    Some(if flipflop.state {
                        Pulse::High
                    } else {
                        Pulse::Low
                    })
                } else {
                    None
                }
            }
            Module::Conjunction(ref mut conj) => {
                assert!(
                    conj.inputs.contains_key(sender),
                    "{sender} not in {:?}",
                    conj.inputs
                );
                *conj.inputs.get_mut(sender).unwrap() = pulse;
                Some(if conj.inputs.values().all(|x| *x == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                })
            }
            Module::Broadcaster => Some(pulse),
            Module::Output => None,
        }
    }

    fn is_reset(&self) -> bool {
        match *self {
            Module::FlipFlop(ref flipflop) => !flipflop.state,
            Module::Conjunction(ref conj) => conj.inputs.values().all(|x| *x == Pulse::Low),
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
    let node_map: HashMap<&str, Node> = text
        .lines()
        .map(|line| {
            let (inp, output) = line.split_once(" -> ").unwrap();
            let output = output.split(", ").collect();
            if let Some(name) = inp.strip_prefix('%') {
                Node {
                    name,
                    module: RefCell::new(Module::FlipFlop(FlipFlop { state: false })),
                    output,
                }
            } else if let Some(name) = inp.strip_prefix('&') {
                Node {
                    name,
                    module: RefCell::new(Module::Conjunction(Conjunction {
                        inputs: HashMap::new(),
                    })),
                    output,
                }
            } else if inp == "broadcaster" {
                Node {
                    name: inp,
                    module: RefCell::new(Module::Broadcaster),
                    output,
                }
            } else if inp == "output" {
                Node {
                    name: inp,
                    module: RefCell::new(Module::Output),
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
                conj.inputs.insert(name, Pulse::Low);
            }
        }
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
            if let Some(node) = &node_map.get(name) {
                let (high_out, low_out) = node.broadcast(sender, Pulse::High);
                high_tmp.extend(high_out.iter().map(|x| (name, *x)));
                low_tmp.extend(low_out.iter().map(|x| (name, *x)));
            }
        }
        for (sender, name) in low_pulses {
            if let Some(node) = &node_map.get(name) {
                let (high_out, low_out) = node.broadcast(sender, Pulse::Low);
                high_tmp.extend(high_out.iter().map(|x| (name, *x)));
                low_tmp.extend(low_out.iter().map(|x| (name, *x)));
            }
        }
        high_count += high_tmp.len() as u64;
        low_count += low_tmp.len() as u64;
        high_pulses = high_tmp
            .into_iter()
            .filter(|(_, n)| *n != "output")
            .collect();
        low_pulses = low_tmp
            .into_iter()
            .filter(|(_, n)| *n != "output")
            .collect();
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

const fn compute2(_text: &str) -> u64 {
    0
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

    #[test]
    fn test_2_first() {
        let result = compute2(INPUT);
        assert_eq!(result, 0);
    }
}
