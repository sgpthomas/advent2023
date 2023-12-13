use std::str::FromStr;

use itertools::Itertools;

use crate::Solution;

pub struct Day9;

#[derive(Debug)]
struct Input(Vec<Sequence>);

#[derive(Debug)]
struct Sequence(Vec<i64>);

impl FromStr for Input {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(
            s.lines().map(|line| line.parse().unwrap()).collect_vec(),
        ))
    }
}

impl FromStr for Sequence {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Sequence(
            s.split_whitespace()
                .map(|a| a.parse().unwrap())
                .collect_vec(),
        ))
    }
}

impl Sequence {
    fn diff_seq(&self) -> Sequence {
        let Sequence(seq) = self;
        Sequence(seq.iter().tuple_windows().map(|(x, y)| y - x).collect_vec())
    }

    fn extrapolate(&self) -> i64 {
        let Sequence(seq) = self;
        if seq.iter().all_equal() {
            seq[0]
        } else {
            let next_layer = self.diff_seq().extrapolate();
            seq[seq.len() - 1] + next_layer
        }
    }

    fn extrapolate_backwards(&self) -> i64 {
        let Sequence(seq) = self;
        if seq.iter().all_equal() {
            seq[0]
        } else {
            let next_layer = self.diff_seq().extrapolate_backwards();
            seq[0] - next_layer
        }
    }
}

impl Solution for Day9 {
    fn part1(input: &str) -> Option<i64> {
        let Input(seq) = input.parse().unwrap();
        Some(seq.iter().map(|s| s.extrapolate()).sum())
    }

    fn part2(input: &str) -> Option<i64> {
        let Input(seq) = input.parse().unwrap();
        Some(seq.iter().map(|s| s.extrapolate_backwards()).sum())
    }
}
