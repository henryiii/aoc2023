#![warn(clippy::all, clippy::pedantic)]

use std::{collections::HashMap, io::prelude::*};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Card {
    value: u64,
}

impl Card {
    fn new(value: u64) -> Self {
        assert!((1..=13).contains(&value));
        Self { value }
    }
}

impl std::str::FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() != 1 || b[0] <= b'1' {
            return Err(());
        }
        match b[0] {
            b'T' => Ok(Self::new(10)),
            b'J' => Ok(Self::new(1)),
            b'Q' => Ok(Self::new(11)),
            b'K' => Ok(Self::new(12)),
            b'A' => Ok(Self::new(13)),
            x if x.is_ascii_digit() => Ok(Self::new((x - b'0').into())),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

fn count(cards: &[Card]) -> HashMap<Card, u64> {
    cards.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(*x).or_insert(0) += 1;
        acc
    })
}

impl Hand {
    fn new(cards: &[Card; 5], bid: u64) -> Self {
        Self { cards: *cards, bid }
    }

    fn level(&self) -> u64 {
        let card_counts = count(&self.cards);
        let mut counts: Vec<_> = card_counts.clone().into_values().collect();
        counts.sort_unstable();
        counts.reverse();

        if let Some(jokers) = card_counts.get(&Card::new(1)) {
            let no_jokers: Vec<Card> = self
                .cards
                .into_iter()
                .filter(|x| *x != Card::new(1))
                .collect();
            let jokerless_card_counts = count(&no_jokers);
            counts = jokerless_card_counts.into_values().collect();
            if counts.is_empty() {
                counts.push(0);
            }
            counts.sort_unstable();
            counts.reverse();
            counts[0] += jokers;
        }

        match counts.as_slice() {
            [5] => 6,
            [4, 1] => 5,
            [3, 2] => 4,
            [3, 1, 1] => 3,
            [2, 2, 1] => 2,
            [2, 1, 1, 1] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => panic!("Invalid hand"),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.level(), self.cards, self.bid).cmp(&(other.level(), other.cards, other.bid))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::str::FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let hand_str = split.next().unwrap();
        let bid_str = split.next().unwrap();

        let cards: Vec<Card> = hand_str
            .chars()
            .map(|x| x.to_string().parse().unwrap())
            .collect();

        let bid: u64 = bid_str.parse().unwrap();
        Ok(Self::new(&cards.try_into().unwrap(), bid))
    }
}

fn main() {
    let file = std::fs::File::open("input/07.txt").unwrap();
    let lines_res = std::io::BufReader::new(file).lines();
    let lines = lines_res.map(std::result::Result::unwrap);
    let mut hands: Vec<Hand> = lines.map(|x| x.parse().unwrap()).collect();
    hands.sort();
    let score: u64 = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum();
    println!("Total winnings: {score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_parse() {
        let lines = INPUT.lines().map(|x| x.to_string());
        let mut hands: Vec<Hand> = lines.map(|x| x.parse().unwrap()).collect();
        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].bid, 684);
        assert_eq!(hands[2].bid, 28);
        assert_eq!(hands[3].bid, 220);
        assert_eq!(hands[4].bid, 483);

        hands.sort();
        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].bid, 28);
        assert_eq!(hands[2].bid, 684);
        assert_eq!(hands[3].bid, 483);
        assert_eq!(hands[4].bid, 220);

        let score: u64 = hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
            .sum();
        assert_eq!(score, 5905)
    }

    #[test]
    fn test_construct() {
        assert_eq!(Card::new(6), "6".parse().unwrap());
        assert_eq!(
            Hand::new(
                &[
                    Card::new(6),
                    Card::new(7),
                    Card::new(8),
                    Card::new(9),
                    Card::new(10)
                ],
                123
            ),
            "6789T 123".parse().unwrap()
        );
    }
}
