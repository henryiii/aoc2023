use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct FailedReadError;

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = FailedReadError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let data = line.split(':').last().ok_or(FailedReadError)?;
        let mut groups = data.split('|');
        let winning_str = groups.next().ok_or(FailedReadError)?.trim();
        let numbers_str = groups.next().ok_or(FailedReadError)?.trim();
        let numbers = numbers_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let winning = winning_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Ok(Self { winning, numbers })
    }
}

impl Card {
    fn count_winning(&self) -> usize {
        self.numbers
            .iter()
            .filter(|x| self.winning.contains(x))
            .count()
    }

    fn score(&self) -> u32 {
        let score = u32::try_from(self.count_winning()).unwrap();
        if score == 0 {
            0
        } else {
            u32::pow(2, score - 1)
        }
    }
}

fn card_count(cards: &[Card]) -> Vec<usize> {
    let wins: Vec<usize> = cards.iter().map(Card::count_winning).collect();
    let mut card_count: Vec<usize> = cards.iter().map(|_| 1).collect();
    for (n, win) in wins.iter().enumerate() {
        for w in (n + 1)..std::cmp::min(*win + n + 1, card_count.len()) {
            card_count[w] += card_count[n];
        }
    }
    card_count
}

fn main() {
    let file = std::fs::File::open("input/04.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let cards: Vec<Card> = lines.map(|x| x.unwrap().parse().unwrap()).collect();
    let score: u32 = cards.iter().map(Card::score).sum();
    println!("Score: {score}");
    let count: usize = card_count(&cards).iter().sum();
    println!("Count: {count:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_03() {
        let lines = INPUT.lines().map(|x| x.to_string());
        let cards: Vec<Card> = lines.map(|x| x.parse().unwrap()).collect();
        assert_eq!(cards.len(), 6);
        assert_eq!(cards[0].winning.len(), 5);
        assert_eq!(cards[0].numbers.len(), 8);
        assert_eq!(cards[0].count_winning(), 4);
        let score: u32 = cards.iter().map(|x| x.score()).sum();
        assert_eq!(score, 13);
        let card_counts = card_count(&cards);
        let count: usize = card_counts.iter().sum();
        assert_eq!(count, 30);
    }
}
