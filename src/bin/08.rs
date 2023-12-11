#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::similar_names)]

use std::collections::HashMap;

fn read(text: &str) -> (String, HashMap<String, (String, String)>) {
    let mut lines = text.lines();
    let directions = lines.next().unwrap().to_string();
    assert!(lines.next().unwrap().is_empty());
    let nodes: HashMap<String, (String, String)> = lines
        .map(|x| {
            let key = x.chars().take(3).collect::<String>();
            let lvalue = x.chars().skip(7).take(3).collect::<String>();
            let rvalue = x.chars().skip(12).take(3).collect::<String>();
            (key, (lvalue, rvalue))
        })
        .collect();
    (directions, nodes)
}

fn follow_directions(directions: &str, nodes: &HashMap<String, (String, String)>) -> u64 {
    let mut current_node = "AAA";
    let mut count = 0;
    for direction in directions.chars().cycle() {
        match direction {
            'L' => current_node = &nodes[current_node].0,
            'R' => current_node = &nodes[current_node].1,
            _ => panic!("Invalid direction: {direction}"),
        }
        count += 1;
        if current_node == "ZZZ" {
            break;
        }
        assert!(count <= 100_000, "Infinite loop detected");
    }
    count
}

fn follow_directions_syml(directions: &str, nodes: &HashMap<String, (String, String)>) -> u64 {
    let mut hm: HashMap<usize, u64> = HashMap::new();
    let mut count = 0;
    let mut current_nodes: Vec<&str> = nodes
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| &x[..])
        .collect();
    println!("{current_nodes:?}");
    for direction in directions.chars().cycle() {
        match direction {
            'L' => current_nodes.iter_mut().for_each(|x| *x = &nodes[*x].0),
            'R' => current_nodes.iter_mut().for_each(|x| *x = &nodes[*x].1),
            _ => panic!("Invalid direction: {direction}"),
        }
        count += 1;
        // For some reason, there is exactly one Z for each node path which
        // occurs on a loop.  So do the dumb thing here and it works. This
        // solution would not work for general maps. But the data is too big to
        // do the general solution.
        if current_nodes.iter().all(|x| x.ends_with('Z')) {
            break;
        }
        for (i, node) in current_nodes.iter().enumerate() {
            if node.ends_with('Z') {
                hm.insert(i, count);
            }
        }
        if hm.len() == current_nodes.len() {
            break;
        }
    }
    hm.into_values().reduce(num::integer::lcm).unwrap()
}

fn main() {
    let text = std::fs::read_to_string("input/08.txt").unwrap();
    let (directions, nodes) = read(&text);
    println!("Number of Directions: {}", directions.len());
    println!("Number of Nodes: {}", nodes.len());

    let count = follow_directions(&directions, &nodes);
    println!("Count: {count}");

    let count = follow_directions_syml(&directions, &nodes);
    println!("Slow Count: {count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    const INPUT2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_01() {
        let (directions, nodes) = read(INPUT1);
        assert_eq!(directions, "RL");
        assert_eq!(nodes["AAA"], ("BBB".to_string(), "CCC".to_string()));
        assert_eq!(nodes["BBB"], ("DDD".to_string(), "EEE".to_string()));
        assert_eq!(nodes["CCC"], ("ZZZ".to_string(), "GGG".to_string()));
        assert_eq!(nodes["DDD"], ("DDD".to_string(), "DDD".to_string()));
        assert_eq!(nodes["EEE"], ("EEE".to_string(), "EEE".to_string()));
        assert_eq!(nodes["GGG"], ("GGG".to_string(), "GGG".to_string()));
        assert_eq!(nodes["ZZZ"], ("ZZZ".to_string(), "ZZZ".to_string()));

        let count = follow_directions(&directions, &nodes);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_02() {
        let (directions, nodes) = read(INPUT2);
        assert_eq!(directions, "LLR");
        assert_eq!(nodes["AAA"], ("BBB".to_string(), "BBB".to_string()));
        assert_eq!(nodes["BBB"], ("AAA".to_string(), "ZZZ".to_string()));
        assert_eq!(nodes["ZZZ"], ("ZZZ".to_string(), "ZZZ".to_string()));

        let count = follow_directions(&directions, &nodes);
        assert_eq!(count, 6);
    }

    #[test]
    fn test_b() {
        let (directions, nodes) = read(INPUT3);
        let start_nodes: Vec<_> = nodes.keys().filter(|x| x.ends_with("A")).collect();
        assert_eq!(start_nodes.len(), 2);
        let count = follow_directions_syml(&directions, &nodes);
        assert_eq!(count, 6);
    }
}
