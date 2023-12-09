use std::io::prelude::*;

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn ways_to_win(&self) -> u64 {
        (0..(self.time))
            .map(|a_time| (a_time * (self.time - a_time) > self.distance) as u64)
            .sum()
    }
}

fn get_arr(string: &str) -> Vec<u64> {
    string
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let file = std::fs::File::open("input/06.txt").unwrap();
    let lines_res = std::io::BufReader::new(file).lines();
    let mut lines = lines_res.map(|x| x.unwrap());
    let time_str = lines.next().unwrap();
    let distance_str = lines.next().unwrap();

    let time: Vec<u64> = get_arr(&time_str);
    let distance: Vec<u64> = get_arr(&distance_str);

    let races: Vec<Race> = time
        .iter()
        .zip(&distance)
        .map(|(t, d)| Race::new(*t, *d))
        .collect();
    let ways_to_win: u64 = races.iter().map(Race::ways_to_win).product();
    println!("Ways to win: {}", ways_to_win);

    let distance_joined: u64 = get_arr(&distance_str.replace(' ', ""))[0];
    let time_joined: u64 = get_arr(&time_str.replace(' ', ""))[0];
    let ways_to_win = Race::new(time_joined, distance_joined).ways_to_win();
    println!("Joined ways to win: {}", ways_to_win);
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
        let time_str = lines.next().unwrap();
        let distance_str = lines.next().unwrap();
        let time: Vec<u64> = get_arr(&time_str);
        let distance: Vec<u64> = get_arr(&distance_str);
        let races: Vec<Race> = time
            .iter()
            .zip(&distance)
            .map(|(t, d)| Race::new(*t, *d))
            .collect();
        let ways_to_win: Vec<u64> = races.iter().map(Race::ways_to_win).collect();
        let total_ways: u64 = ways_to_win.iter().product();

        assert_eq!(time, vec![7, 15, 30]);
        assert_eq!(distance, vec![9, 40, 200]);
        assert_eq!(ways_to_win, vec![4, 8, 9]);
        assert_eq!(total_ways, 288);

        let distance_joined: u64 = get_arr(&distance_str.replace(" ", ""))[0];
        let time_joined: u64 = get_arr(&time_str.replace(" ", ""))[0];
        let ways_to_win = Race::new(time_joined, distance_joined).ways_to_win();
        assert_eq!(ways_to_win, 71503);
    }
}
