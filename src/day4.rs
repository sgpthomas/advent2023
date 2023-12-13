use std::{collections::VecDeque, str::FromStr};

use itertools::Itertools;

use crate::Solution;

pub struct Day4;

#[derive(Debug)]
struct Game {
    cards: Vec<Card>,
    worklist: VecDeque<usize>,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<_> = s.lines().map(|line| line.parse().unwrap()).collect();
        let worklist: VecDeque<_> = (0..cards.len()).collect();
        Ok(Game { cards, worklist })
    }
}

#[derive(Debug)]
struct Card {
    id: usize,
    lucky_numbers: Vec<usize>,
    drawn_numbers: Vec<usize>,
}

impl FromStr for Card {
    type Err = &'static str;

    /// Input like:
    ///  'Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53'
    ///       id  lucky_numbers    drawn_numbers
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split([':', '|']).collect_vec().as_slice() {
            [id, lucky, drawn] => Ok(Card {
                // subtract 1 from the id so that the ids match the vec index
                id: (id.split_whitespace().collect_vec()[1]
                    .parse::<usize>()
                    .expect("invalid id parsing"))
                    - 1,
                lucky_numbers: lucky
                    .trim()
                    .split_whitespace()
                    .map(|num| num.parse().expect(&format!("{num}")))
                    .collect_vec(),
                drawn_numbers: drawn
                    .trim()
                    .split_whitespace()
                    .map(|num| num.parse().expect(&format!("{num}")))
                    .collect_vec(),
            }),
            _ => Err("Malformed input"),
        }
    }
}

impl Card {
    fn matching_cards(&self) -> usize {
        self.drawn_numbers
            .iter()
            .map(|num| (num, self.lucky_numbers.contains(num) as usize))
            .unique_by(|(id, _)| *id)
            .filter(|(_, v)| *v == 1)
            .count()
    }

    fn score(&self) -> usize {
        let n = self.matching_cards();

        if n == 0 {
            0
        } else {
            usize::pow(2, (n - 1) as u32)
        }
    }
}

impl Game {
    fn populate_cache(&self) -> Vec<usize> {
        self.cards
            .iter()
            .map(|card| card.matching_cards())
            .collect()
    }

    fn explode(&mut self) -> usize {
        let mut total = 0;
        let cache = self.populate_cache();
        while let Some(card_id) = self.worklist.pop_front() {
            total += 1;
            let card = &self.cards[card_id];
            let wins = cache[card_id];
            if wins > 0 {
                for next_id in 0..wins {
                    self.worklist.push_back(card.id + next_id + 1)
                }
            }
        }
        total
    }
}

impl Solution for Day4 {
    fn part1(input: &str) -> Option<i64> {
        let game: Game = input.parse().unwrap();
        Some(game.cards.iter().map(|x| x.score()).sum::<usize>() as i64)
    }

    fn part2(input: &str) -> Option<i64> {
        let mut game: Game = input.parse().unwrap();
        Some(game.explode() as i64)
    }
}
