/*!
# 2023 Day 7: Camel Cards
## Poker-like cards

<https://adventofcode.com/2023/day/7>

This is a poker-like game, just simpler. I'm using `strum` to handle the cards
as an enum (see history for a pure struct implementation, also the removed
`07b`).  This is the simpler version with a single set of cards - in 07-trait,
I've implemented a Card trait and two types of cards in order to handle the
rules for different games.  That's much more interesting Rust, so I'd suggest
looking at that one next.

To compute the hand level, I'm using a `HashMap`. In Python, I'd have used a
`Counter`.
*/

use core::fmt::Debug;
use core::hash::Hash;
use core::str::FromStr;
use std::cmp::Eq;

use std::collections::HashMap;

use derive_more::Constructor;
use itertools::Itertools;
use strum::EnumString;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, EnumString)]
enum Card {
    #[strum(serialize = "?")]
    Joker,
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    #[strum(serialize = "T")]
    Ten,
    #[strum(serialize = "J")]
    Jack,
    #[strum(serialize = "Q")]
    Queen,
    #[strum(serialize = "K")]
    King,
    #[strum(serialize = "A")]
    Ace,
}

#[derive(Debug, PartialEq, Eq, Constructor)]
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
    fn level(&self) -> u64 {
        let card_counts = count(&self.cards);
        let mut counts: Vec<_> = card_counts.clone().into_values().collect();
        counts.sort_unstable();
        counts.reverse();

        let jokers: u64 = card_counts
            .iter()
            .filter(|(k, _)| *k == &Card::Joker)
            .map(|(_, v)| v)
            .sum();
        if jokers > 0 {
            let no_jokers: Vec<_> = self
                .cards
                .into_iter()
                .filter(|x| x != &Card::Joker)
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

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bid_str) = s.split_ascii_whitespace().collect_tuple().unwrap();

        let cards: Vec<Card> = hand_str
            .chars()
            .map(|x| x.to_string().parse().unwrap())
            .collect();

        let bid: u64 = bid_str.parse().unwrap();
        Ok(Self::new(cards.try_into().unwrap(), bid))
    }
}

fn main() {
    let text = std::fs::read_to_string("input/07.txt").unwrap();
    let mut hands: Vec<Hand> = text.lines().map(|x| x.parse().unwrap()).collect();
    hands.sort();
    let score: u64 = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum();
    println!("Total winnings: {score}");

    let mut hands: Vec<Hand> = text
        .lines()
        .map(|x| x.replace('J', "?").parse().unwrap())
        .collect();
    hands.sort();
    let score: u64 = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum();
    println!("Total winnings using Jokers: {score}");
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
        let lines = INPUT.lines().map(std::string::ToString::to_string);
        let mut hands: Vec<Hand> = lines.map(|x| x.parse().unwrap()).collect();
        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].bid, 684);
        assert_eq!(hands[2].bid, 28);
        assert_eq!(hands[3].bid, 220);
        assert_eq!(hands[4].bid, 483);

        hands.sort();
        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].bid, 220);
        assert_eq!(hands[2].bid, 28);
        assert_eq!(hands[3].bid, 684);
        assert_eq!(hands[4].bid, 483);

        let score: u64 = hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
            .sum();
        assert_eq!(score, 6440);
    }

    #[test]
    fn test_parse_2() {
        let lines = INPUT.lines().map(std::string::ToString::to_string);
        let mut hands: Vec<Hand> = lines
            .map(|x| x.replace('J', "?").parse().unwrap())
            .collect();
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
        assert_eq!(score, 5905);
    }

    #[test]
    fn test_construct() {
        assert_eq!(Card::Six, "6".parse().unwrap());
        assert_eq!(
            Hand::new(
                [Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten],
                123
            ),
            "6789T 123".parse().unwrap()
        );
    }
}
