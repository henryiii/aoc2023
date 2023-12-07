use std::io::prelude::*;

struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn new(time: u32, distance: u32) -> Self {
        Self { time, distance }
    }

    fn ways_to_win(&self) -> u32 {
        (0..(self.time))
            .map(|a_time| {
                if a_time * (self.time - a_time) > self.distance {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>()
    }
}

fn main() {
    let file = std::fs::File::open("input/06.txt").unwrap();
    let lines_res = std::io::BufReader::new(file).lines();
    let mut lines = lines_res.map(|x| x.unwrap());
    let time: Vec<u32> = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let distance: Vec<u32> = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let races: Vec<Race> = time
        .iter()
        .zip(&distance)
        .map(|(t, d)| Race::new(*t, *d))
        .collect();
    let ways_to_win: u32 = races.iter().map(Race::ways_to_win).product();
    println!("{}", ways_to_win);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_read() {
        let mut lines = INPUT.lines().map(|x| x.to_string());
        let time: Vec<u32> = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let distance: Vec<u32> = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let races: Vec<Race> = time
            .iter()
            .zip(&distance)
            .map(|(t, d)| Race::new(*t, *d))
            .collect();
        let ways_to_win: Vec<u32> = races.iter().map(Race::ways_to_win).collect();
        let total_ways: u32 = ways_to_win.iter().product();

        assert_eq!(time, vec![7, 15, 30]);
        assert_eq!(distance, vec![9, 40, 200]);
        assert_eq!(ways_to_win, vec![4, 8, 9]);
        assert_eq!(total_ways, 288);
    }
}
