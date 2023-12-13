use std::{cmp, str::FromStr};

use itertools::Itertools;

use crate::Solution;

pub struct Day2;

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    draws: Vec<(usize, Color)>,
}

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split_at(s.find(": ").expect("No colon"));
        let id = &id[5..]; // ignore the 'Game ' text at the beginning of id
        let rounds = rest[2..] // ignore the ': ' text at the beginning
            .split("; ")
            .map(|round| round.parse::<Round>().unwrap())
            .collect::<Vec<_>>();
        Ok(Game {
            id: id.parse().expect("Not a number"),
            rounds,
        })
    }
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let draws = s
            .split(", ")
            .map(|x| {
                x.split_whitespace()
                    .tuples()
                    .map(|(number, color)| (number.parse().unwrap(), color.parse().unwrap()))
                    .collect::<Vec<(usize, Color)>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        Ok(Self { draws })
    }
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            _ => Err("Unknown Color"),
        }
    }
}

impl Game {
    fn min_possible(&self) -> (usize, usize, usize) {
        self.rounds.iter().fold((0, 0, 0), |(r, g, b), round| {
            let (rc, gc, bc) = round.counts();
            (cmp::max(r, rc), cmp::max(g, gc), cmp::max(b, bc))
        })
    }

    fn possible_with(&self, red: usize, green: usize, blue: usize) -> bool {
        self.rounds
            .iter()
            .all(|r| r.possible_with(red, green, blue))
    }
}

impl Round {
    fn counts(&self) -> (usize, usize, usize) {
        self.draws
            .iter()
            .fold((0, 0, 0), |(r, g, b), (count, color)| match color {
                Color::Red => (r + count, g, b),
                Color::Green => (r, g + count, b),
                Color::Blue => (r, g, b + count),
            })
    }

    fn possible_with(&self, red: usize, green: usize, blue: usize) -> bool {
        let (red_count, green_count, blue_count) = self.counts();
        red_count <= red && green_count <= green && blue_count <= blue
    }
}

impl Solution for Day2 {
    fn part1(input: &str) -> Option<i64> {
        Some(
            input
                .lines()
                .map(|line| line.parse::<Game>().unwrap())
                .filter_map(|game| game.possible_with(12, 13, 14).then_some(game.id))
                .sum::<usize>() as i64,
        )
    }

    fn part2(input: &str) -> Option<i64> {
        Some(
            input
                .lines()
                .map(|line| line.parse::<Game>().unwrap())
                .map(|game| {
                    let (r, g, b) = game.min_possible();
                    r * g * b
                })
                .sum::<usize>() as i64,
        )
    }
}
