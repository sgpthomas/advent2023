use std::str::FromStr;

use itertools::Itertools;

use crate::Solution;

pub struct Day6;

struct Quadratic {
    a: f64,
    b: f64,
    c: f64,
}

impl Quadratic {
    fn new(a: f64, b: f64, c: f64) -> Self {
        Quadratic { a, b, c }
    }

    fn solve(&self) -> (f64, f64) {
        let radical = f64::sqrt(self.b.powi(2) - (4. * self.a * self.c));
        (
            (-self.b + radical) / (2. * self.a),
            (-self.b - radical) / (2. * self.a),
        )
    }
}

/// Wrapper type to hold part 1 races
#[derive(Debug)]
struct Races(Vec<Race>);

/// Wrapper type to hold part 2
#[derive(Debug)]
struct LongRace(Race);

#[derive(Debug)]
struct Race {
    total_time: usize,
    best_distance: usize,
}

// == parsing ==
impl FromStr for Races {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Races(
            s.split('\n')
                .map(|line| {
                    line.split_whitespace()
                        .skip(1)
                        .map(|n| n.parse::<usize>().unwrap())
                })
                .tuples()
                .map(|(time, distance)| {
                    time.zip(distance).map(|(total_time, best_distance)| Race {
                        total_time,
                        best_distance,
                    })
                })
                .flatten()
                .collect_vec(),
        ))
    }
}

impl FromStr for LongRace {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LongRace(
            s.split('\n')
                .map(|line| {
                    line.split_whitespace()
                        .skip(1)
                        .join("")
                        .parse::<usize>()
                        .unwrap()
                })
                .tuples()
                .map(|(total_time, best_distance)| Race {
                    total_time,
                    best_distance,
                })
                .next()
                .unwrap(),
        ))
    }
}

impl Race {
    fn winning_range(&self) -> (usize, usize) {
        let (sol1, sol2) =
            Quadratic::new(-1., self.total_time as f64, -(self.best_distance as f64)).solve();

        // find the nearest integer solution
        ((sol1 + 1.0).floor() as usize, (sol2 - 1.0).ceil() as usize)
    }
}

impl Solution for Day6 {
    fn part1(input: &str) -> Option<usize> {
        let Races(races) = input.parse().unwrap();
        Some(
            races
                .iter()
                .map(|r| r.winning_range())
                .map(|(l, u)| u - l + 1)
                .product(),
        )
    }

    fn part2(input: &str) -> Option<usize> {
        let LongRace(race) = input.parse().unwrap();
        let (l, u) = race.winning_range();
        Some(u - l + 1)
    }
}
