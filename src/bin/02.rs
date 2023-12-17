/*!
# 2023 Day 2 - Colored balls

<https://adventofcode.com/2023/day/2>

This is a nice introduction to custom structs. I originally used pure Rust (see
history), but added some nice utilities from `derive_more` and `itertools` to
simplify (pretty much fully replace) the impl blocks. I'm also implementing the
`FromStr` trait to convert from a string to my Meas struct.

I'm also doing some error handling here (see history for different version, also
with error handling). It was rather forced on me by the `FromStr` trait, but I
think it's a good idea to get used to it instead of simply unwrapping all the
time. Which I'm going to most of the time, since these are controlled inputs.

I'm currently not using `anyhow` and `thiserror`, but might in the future. This
would make error handling and messages better, and avoid the
`.or(Err(FailedReadError))`.
*/

use derive_more::{Add, Constructor};
use itertools::Itertools;
use std::str::FromStr;

/// This is an error for my `FromStr` implementation.
#[derive(Debug, Clone)]
struct FailedReadError;

/// This is a measurement of balls, with the number of each color present.
#[derive(Default, Add, Constructor)]
struct Meas {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Meas {
    type Err = FailedReadError;

    /// This accepts strings of the form `"1 red, 2 green, 3 blue"` and adds
    /// them together.
    fn from_str(meas: &str) -> Result<Self, Self::Err> {
        meas.trim().split(',').try_fold(Self::default(), |acc, x| {
            let (val, color) = x
                .split_ascii_whitespace()
                .collect_tuple()
                .ok_or(FailedReadError)?;
            let val: u32 = val.parse().or(Err(FailedReadError))?;
            let current = match color {
                "red" => Self::new(val, 0, 0),
                "green" => Self::new(0, val, 0),
                "blue" => Self::new(0, 0, val),
                _ => Err(FailedReadError)?,
            };
            Ok(acc + current)
        })
    }
}

fn measurements(line: &str) -> (u32, Vec<Meas>) {
    let split: Vec<&str> = line.split(':').collect();
    assert_eq!(split.len(), 2);
    let game_str = split[0];
    let game_number: u32 = game_str.strip_prefix("Game ").unwrap().parse().unwrap();
    let results = split[1].split(';');
    let meas = results.map(|x| x.parse().unwrap()).collect();
    (game_number, meas)
}

fn valid_measurements(max: &Meas, all_meas: &[Meas]) -> bool {
    all_meas
        .iter()
        .all(|x| max.red >= x.red && max.blue >= x.blue && max.green >= x.green)
}

fn accumulator(acc: u32, line: &str) -> u32 {
    let (game_number, all_meas) = measurements(line);
    if valid_measurements(&Meas::new(12, 13, 14), &all_meas) {
        acc + game_number
    } else {
        acc
    }
}

fn total_power(line: &str) -> u32 {
    let (_, all_meas) = measurements(line);
    let total = all_meas.iter().fold(Meas::default(), |acc, x| {
        Meas::new(
            std::cmp::max(acc.red, x.red),
            std::cmp::max(acc.green, x.green),
            std::cmp::max(acc.blue, x.blue),
        )
    });
    total.red * total.green * total.blue
}

fn main() {
    let text = std::fs::read_to_string("input/02.txt").unwrap();
    let sum = text.lines().fold(0, accumulator);

    let pow = text.lines().fold(0, |acc, x| acc + total_power(x));
    println!("Sum: {sum}");
    println!("Total power: {pow}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_02() {
        let full_total = INPUT.lines().fold(0, |acc, x| acc + measurements(x).0);
        assert_eq!(full_total, 15);
        let sum = INPUT.lines().fold(0, |acc, x| accumulator(acc, x));
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_02b() {
        let pow = INPUT.lines().fold(0, |acc, x| acc + total_power(x));
        assert_eq!(pow, 2286);
    }
}
