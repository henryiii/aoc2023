use std::io::prelude::*;
use std::ops::Add;

#[derive(Default)]
struct Meas {
    red: u32,
    blue: u32,
    green: u32,
}

impl Meas {
    fn from_str(meas: &str) -> Meas {
        if let Some(val) = meas.strip_suffix(" red") {
            Meas {
                red: val.parse().unwrap(),
                ..Default::default()
            }
        } else if let Some(val) = meas.strip_suffix(" blue") {
            Meas {
                blue: val.parse().unwrap(),
                ..Default::default()
            }
        } else if let Some(val) = meas.strip_suffix(" green") {
            Meas {
                green: val.parse().unwrap(),
                ..Default::default()
            }
        } else {
            panic!("Can't parse {meas}")
        }
    }
}

impl Add for Meas {
    type Output = Meas;

    fn add(self, other: Meas) -> Meas {
        Meas {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

fn make_meas(meas_str: &str) -> Meas {
    meas_str.trim().split(", ").fold(
        Meas {
            ..Default::default()
        },
        |acc, x| acc + Meas::from_str(x),
    )
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

fn valid_measurements(max: Meas, all_meas: &[Meas]) -> bool {
    all_meas
        .iter()
        .all(|x| max.red >= x.red && max.blue >= x.blue && max.green >= x.green)
}

fn accumulator(acc: u32, line: &str) -> u32 {
    let (game_number, all_meas) = measurements(line);
    if valid_measurements(
        Meas {
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
    let total = all_meas.iter().fold(
        Meas {
            ..Default::default()
        },
        |acc, x| Meas {
            red: std::cmp::max(acc.red, x.red),
            green: std::cmp::max(acc.green, x.green),
            blue: std::cmp::max(acc.blue, x.blue),
        },
    );
    total.red * total.green * total.blue
}

fn main() {
    let file = std::fs::File::open("resources/02/input.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let sum = lines.fold(0, |acc, x| accumulator(acc, &x.unwrap()));

    let file = std::fs::File::open("resources/02/input.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let pow = lines.fold(0, |acc, x| acc + total_power(&x.unwrap()));
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
        let lines = INPUT.lines();
        let full_total = lines.clone().fold(0, |acc, x| acc + measurements(x).0);
        assert_eq!(full_total, 15);
        let sum = lines.fold(0, |acc, x| accumulator(acc, x));
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_02b() {
        let lines = INPUT.lines();
        let pow = lines.fold(0, |acc, x| acc + total_power(x));
        assert_eq!(pow, 2286);
    }
}