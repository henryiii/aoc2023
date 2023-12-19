/*!
# 2023 Day 19 - Command queues

<https://adventofcode.com/2023/day/19>

This uses structs and enums to represent the rules and workflows.

*/

use gcollections::ops::{set::Intersection, Cardinality, Difference};
use interval::{interval_set::ToIntervalSet, IntervalSet};
use regex::{Regex, RegexBuilder};
use std::{collections::HashMap, str::FromStr};
use strum::EnumString;

#[derive(Debug, EnumString, PartialEq, Eq, Hash, Copy, Clone)]
#[strum(serialize_all = "lowercase")]
enum Cat {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, Clone)]
struct PartRange {
    x: IntervalSet<u64>,
    m: IntervalSet<u64>,
    a: IntervalSet<u64>,
    s: IntervalSet<u64>,
}

impl PartRange {
    fn sum(&self) -> u64 {
        self.x.iter().map(Cardinality::size).sum::<u64>()
            * self.m.iter().map(Cardinality::size).sum::<u64>()
            * self.a.iter().map(Cardinality::size).sum::<u64>()
            * self.s.iter().map(Cardinality::size).sum::<u64>()
    }
}

#[derive(Debug)]
enum Destination {
    Workflow(String),
    Accept,
    Reject,
}

impl FromStr for Destination {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            _ => Ok(Self::Workflow(s.to_string())),
        }
    }
}

#[derive(Debug)]
enum Compare {
    LessThan(u64),
    GreaterThan(u64),
}

impl Compare {
    const fn compare(&self, num: u64) -> bool {
        match self {
            Self::LessThan(n) => num < *n,
            Self::GreaterThan(n) => num > *n,
        }
    }
}

impl FromStr for Compare {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, num) = s.split_at(1);
        let num = num.parse().unwrap();
        match op {
            "<" => Ok(Self::LessThan(num)),
            ">" => Ok(Self::GreaterThan(num)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Rule {
    cat: Cat,
    compare: Compare,
    dest: Destination,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    dest: Destination,
}

impl Workflow {
    fn next(&self, part: &Part) -> &Destination {
        for rule in &self.rules {
            if rule.cat == Cat::X && rule.compare.compare(part.x) {
                return &rule.dest;
            }
            if rule.cat == Cat::M && rule.compare.compare(part.m) {
                return &rule.dest;
            }
            if rule.cat == Cat::A && rule.compare.compare(part.a) {
                return &rule.dest;
            }
            if rule.cat == Cat::S && rule.compare.compare(part.s) {
                return &rule.dest;
            }
        }
        &self.dest
    }
}

fn read_workflows(txt: &str) -> HashMap<String, Workflow> {
    let workflow_regex = RegexBuilder::new(
        r"
      ([[:alpha:]]+)         # workflow name
        \{                     # opening brace
            ([^}]+)            # rules
            ,                  # separator
        ([[:alpha:]]+)       # destination
        \}                     # closing brace
      ",
    )
    .ignore_whitespace(true)
    .build()
    .unwrap();
    let rules_regex = RegexBuilder::new(
        r"
            ([xmas])           # rule
            ([><]\d+)          # comparison
            :                  # separator
            ([[:alpha:]]+)   # destination
      ",
    )
    .ignore_whitespace(true)
    .build()
    .unwrap();

    workflow_regex
        .captures_iter(txt)
        .map(|cap| {
            let (_, [name, rules, dest]) = cap.extract();
            let rules = rules_regex
                .captures_iter(rules)
                .map(|cap| {
                    let (_, [rule, comp, dest]) = cap.extract();
                    let cat = Cat::from_str(rule).unwrap();
                    let compare = comp.parse().unwrap();
                    let dest = dest.parse().unwrap();
                    Rule { cat, compare, dest }
                })
                .collect();
            let dest = dest.parse().unwrap();
            (name.to_string(), Workflow { rules, dest })
        })
        .collect()
}

fn read_parts(txt: &str) -> Vec<Part> {
    let part_regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    part_regex
        .captures_iter(txt)
        .map(|cap| {
            let (_, [x, m, a, s]) = cap.extract();
            Part {
                x: x.parse().unwrap(),
                m: m.parse().unwrap(),
                a: a.parse().unwrap(),
                s: s.parse().unwrap(),
            }
        })
        .collect()
}

fn read_both(txt: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (wf, parts) = txt.split_once("\n\n").unwrap();
    let wf = read_workflows(wf);
    let parts = read_parts(parts);
    assert!(!wf.is_empty());
    assert!(!parts.is_empty());
    (wf, parts)
}

fn compute1(text: &str) -> u64 {
    let (workflows, parts) = read_both(text);
    parts
        .iter()
        .map(|p| {
            let mut wf = workflows.get("in").unwrap();
            loop {
                let dest = wf.next(p);
                match dest {
                    Destination::Accept => return p.x + p.m + p.a + p.s,
                    Destination::Reject => return 0,
                    Destination::Workflow(name) => wf = workflows.get(name).unwrap(),
                }
            }
        })
        .sum()
}

fn compute2(text: &str) -> u64 {
    let (workflows, _) = read_both(text);
    let parts = PartRange {
        x: vec![(1, 4000)].to_interval_set(),
        m: vec![(1, 4000)].to_interval_set(),
        a: vec![(1, 4000)].to_interval_set(),
        s: vec![(1, 4000)].to_interval_set(),
    };
    accepted_in_part_range(&workflows, workflows.get("in").unwrap(), parts)
}

fn accepted_in_part_range(
    workflows: &HashMap<String, Workflow>,
    wf: &Workflow,
    parts: PartRange,
) -> u64 {
    let mut parts = parts;
    let mut total = 0;

    for rule in &wf.rules {
        let range = match rule.cat {
            Cat::X => &parts.x,
            Cat::M => &parts.m,
            Cat::A => &parts.a,
            Cat::S => &parts.s,
        };
        let rule_range = match rule.compare {
            Compare::LessThan(n) => vec![(0, n - 1)].to_interval_set(),
            Compare::GreaterThan(n) => vec![(n + 1, 4000)].to_interval_set(),
        };
        let intersection = range.intersection(&rule_range);
        let rule_ranges = match rule.cat {
            Cat::X => PartRange {
                x: intersection,
                ..parts.clone()
            },
            Cat::M => PartRange {
                m: intersection,
                ..parts.clone()
            },
            Cat::A => PartRange {
                a: intersection,
                ..parts.clone()
            },
            Cat::S => PartRange {
                s: intersection,
                ..parts.clone()
            },
        };
        match &rule.dest {
            Destination::Accept => {
                total += rule_ranges.sum();
            }
            Destination::Reject => {}
            Destination::Workflow(name) => {
                let wf = workflows.get(name).unwrap();
                total += accepted_in_part_range(workflows, wf, rule_ranges);
            }
        }

        let remaining = range.difference(&rule_range);

        parts = match rule.cat {
            Cat::X => PartRange {
                x: remaining,
                ..parts.clone()
            },
            Cat::M => PartRange {
                m: remaining,
                ..parts.clone()
            },
            Cat::A => PartRange {
                a: remaining,
                ..parts.clone()
            },
            Cat::S => PartRange {
                s: remaining,
                ..parts.clone()
            },
        };
    }

    match &wf.dest {
        Destination::Accept => {
            total += parts.sum();
        }
        Destination::Reject => {}
        Destination::Workflow(name) => {
            let wf = workflows.get(name).unwrap();
            total += accepted_in_part_range(workflows, wf, parts);
        }
    }
    total
}

fn main() {
    let text = std::fs::read_to_string("input/19.txt").unwrap();
    let result = compute1(&text);
    println!("First = {result}");

    let result = compute2(&text);
    println!("Second = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_first() {
        let result = compute1(INPUT);
        assert_eq!(result, 19114);
    }

    #[test]
    fn test_second() {
        let result = compute2(INPUT);
        assert_eq!(result, 167409079868000);
    }
}
