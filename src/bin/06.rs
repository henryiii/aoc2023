use derive_more::Constructor;
use itertools::Itertools;

#[derive(Constructor)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        (0..(self.time))
            .map(|a_time| u64::from(a_time * (self.time - a_time) > self.distance))
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
    let text = std::fs::read_to_string("input/06.txt").unwrap();
    let (time, distance) = text.lines().map(get_arr).collect_tuple().unwrap();

    let races: Vec<Race> = time
        .iter()
        .zip(&distance)
        .map(|(t, d)| Race::new(*t, *d))
        .collect();
    let ways_to_win: u64 = races.iter().map(Race::ways_to_win).product();
    println!("Ways to win: {ways_to_win}");

    let (time, distance) = text
        .lines()
        .map(|x| get_arr(&x.replace(' ', ""))[0])
        .collect_tuple()
        .unwrap();
    let ways_to_win = Race::new(time, distance).ways_to_win();
    println!("Joined ways to win: {ways_to_win}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_read() {
        let (time, distance) = INPUT.lines().map(get_arr).collect_tuple().unwrap();
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

        let (time, distance) = INPUT
            .lines()
            .map(|x| get_arr(&x.replace(' ', ""))[0])
            .collect_tuple()
            .unwrap();
        let ways_to_win = Race::new(time, distance).ways_to_win();
        assert_eq!(ways_to_win, 71503);
    }
}
