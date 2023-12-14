use derive_more::{Add, Constructor};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct FailedReadError;

#[derive(Default, Add, Constructor)]
struct Meas {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Meas {
    type Err = FailedReadError;

    fn from_str(meas: &str) -> Result<Self, Self::Err> {
        match meas.split_ascii_whitespace().collect_tuple() {
            Some((val, "red")) => Ok(Self::new(val.parse().unwrap(), 0, 0)),
            Some((val, "green")) => Ok(Self::new(0, val.parse().unwrap(), 0)),
            Some((val, "blue")) => Ok(Self::new(0, 0, val.parse().unwrap())),
            _ => Err(FailedReadError),
        }
    }
}

fn make_meas(meas_str: &str) -> Meas {
    meas_str
        .trim()
        .split(", ")
        .fold(Meas::default(), |acc, x| acc + x.parse().unwrap())
}

fn measurements(line: &str) -> (u32, Vec<Meas>) {
    let split: Vec<&str> = line.split(':').collect();
    assert_eq!(split.len(), 2);
    let game_str = split[0];
    let game_number: u32 = game_str.strip_prefix("Game ").unwrap().parse().unwrap();
    let results = split[1].split(';');
    let meas = results.map(make_meas).collect();
    (game_number, meas)
}

fn valid_measurements(max: &Meas, all_meas: &[Meas]) -> bool {
    all_meas
        .iter()
        .all(|x| max.red >= x.red && max.blue >= x.blue && max.green >= x.green)
}

fn accumulator(acc: u32, line: &str) -> u32 {
    let (game_number, all_meas) = measurements(line);
    if valid_measurements(
        &Meas {
            red: 12,
            green: 13,
            blue: 14,
        },
        &all_meas[..],
    ) {
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
