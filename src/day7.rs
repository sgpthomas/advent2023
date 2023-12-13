use std::{cmp::Ordering, fmt::Display, str::FromStr};

use itertools::Itertools;

use crate::Solution;

pub struct Day7;

#[derive(Debug)]
struct Game(Vec<(Hand, usize)>);

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

/// Represents all cards as numbers from 2-14
/// 2-9 just represent normal number cards.
/// 10 -> T
/// 11 -> J
/// 12 -> Q
/// 13 -> K
/// 14 -> A
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card(usize);

#[derive(Clone, Debug, PartialEq, Eq)]
enum Type {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Type {
    fn disriminant_id(&self) -> usize {
        match self {
            Type::FiveKind => 6,
            Type::FourKind => 5,
            Type::FullHouse => 4,
            Type::ThreeKind => 3,
            Type::TwoPair => 2,
            Type::OnePair => 1,
            Type::HighCard => 0,
        }
    }

    /// Use 1 joker to upgrade your type
    fn upgrade(&self, n_jokers: usize) -> Type {
        match (self, n_jokers) {
            // no jokers
            (_, 0) => self.clone(),

            (Type::FiveKind, 5) => Type::FiveKind,
            (Type::FiveKind, _) => unreachable!(),

            // 4, 1
            (Type::FourKind, 4) | (Type::FourKind, 1) => Type::FiveKind,
            (Type::FourKind, _) => unreachable!(),

            // 3, 2
            (Type::FullHouse, 3) | (Type::FullHouse, 2) => Type::FiveKind,
            (Type::FullHouse, _) => unreachable!(),

            // 3, 1, 1
            (Type::ThreeKind, 3) | (Type::ThreeKind, 1) => Type::FourKind,
            (Type::ThreeKind, _) => unreachable!(),

            // 2, 2, 1
            (Type::TwoPair, 2) => Type::FourKind,
            (Type::TwoPair, 1) => Type::FullHouse,
            (Type::TwoPair, _) => unreachable!(),

            // 2, 1, 1, 1
            (Type::OnePair, 2) | (Type::OnePair, 1) => Type::ThreeKind,
            (Type::OnePair, _) => unreachable!(),

            // 1, 1, 1, 1, 1
            (Type::HighCard, 1) => Type::OnePair,
            (Type::HighCard, _) => unreachable!(),
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.disriminant_id().partial_cmp(&other.disriminant_id())
    }
}

// Display code

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d, e] = &self.cards;
        write!(f, "{a}{b}{c}{d}{e}")
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card(x) if x < &10 => write!(f, "{x}"),
            Card(10) => write!(f, "T"),
            Card(11) => write!(f, "J"),
            Card(12) => write!(f, "Q"),
            Card(13) => write!(f, "K"),
            Card(14) => write!(f, "A"),
            _ => Err(std::fmt::Error),
        }
    }
}

// == Parsing ==
impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Game(
            s.lines()
                .flat_map(|line| {
                    line.split_whitespace()
                        .tuples()
                        .map(|(hand, bid)| (hand.parse().unwrap(), bid.parse().unwrap()))
                        .collect_vec()
                })
                .collect_vec(),
        ))
    }
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().collect_vec().as_slice() {
            [a, b, c, d, e] => Ok(Hand {
                cards: [
                    (*a).into(),
                    (*b).into(),
                    (*c).into(),
                    (*d).into(),
                    (*e).into(),
                ],
            }),
            _ => Err("Wrong sized hand"),
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            c if c.is_numeric() => Card(c.to_digit(10).unwrap() as usize),
            'T' => Card(10),
            'J' => Card(11),
            'Q' => Card(12),
            'K' => Card(13),
            'A' => Card(14),
            _ => unreachable!(),
        }
    }
}

// == Solution code ==

impl Hand {
    fn type_of(&self) -> Type {
        let mut bins = [0; 15];
        for Card(c) in &self.cards {
            bins[*c] += 1;
        }
        let n_jokers = bins[1];
        let key = bins
            .into_iter()
            .filter(|x| *x > 0)
            .sorted()
            .rev()
            .collect_vec();
        let typ = match key.as_slice() {
            [5] => Type::FiveKind,
            [4, 1] => Type::FourKind,
            [3, 2] => Type::FullHouse,
            [3, 1, 1] => Type::ThreeKind,
            [2, 2, 1] => Type::TwoPair,
            [2, 1, 1, 1] => Type::OnePair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            _ => unreachable!(),
        };
        typ.upgrade(n_jokers)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.type_of().partial_cmp(&other.type_of()) {
            Some(Ordering::Equal) => Some(self.cards.iter().zip(other.cards.iter()).fold(
                Ordering::Equal,
                |acc, (c1, c2)| match acc {
                    Ordering::Equal => c1.cmp(c2),
                    _ => acc,
                },
            )),
            Some(x) => Some(x),
            None => None,
        }
    }
}

impl Solution for Day7 {
    fn part1(input: &str) -> Option<usize> {
        let Game(games) = input.parse().unwrap();
        Some(
            games
                .iter()
                .sorted_by(|(h1, _), (h2, _)| h1.partial_cmp(h2).unwrap())
                .enumerate()
                .map(|(rank, (h, bid))| (rank + 1) * bid)
                .sum(),
        )
    }

    fn part2(input: &str) -> Option<usize> {
        println!("");
        println!("== part 2 ==");
        // we are going to replace all instances of J with 1 which
        // will represent the joker. this will automatically handle
        // the secondary ranking
        let Game(games) = input.replace('J', "1").parse().unwrap();
        Some(
            games
                .iter()
                .sorted_by(|(h1, _), (h2, _)| h1.partial_cmp(h2).unwrap())
                .enumerate()
                .map(|(rank, (h, bid))| (rank + 1) * bid)
                .sum(),
        )
    }
}
