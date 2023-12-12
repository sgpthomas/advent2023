use std::{collections::VecDeque, str::FromStr};

use indicatif::{
    ParallelProgressIterator, ProgressBar, ProgressFinish, ProgressIterator, ProgressStyle,
};
use itertools::Itertools;
use rayon::{
    iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator},
    prelude::ParallelBridge,
    slice::ParallelSlice,
    ThreadPoolBuilder,
};

use crate::Solution;

pub struct Day5;

struct Input(Vec<usize>, Almanac);

#[derive(Debug)]
struct Almanac {
    maps: VecDeque<(String, Map)>,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    dest_start: usize,
    src_start: usize,
    length: usize,
}

// ==== Parsing Code ====

impl FromStr for Input {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections = s.split("\n\n").collect_vec();

        if let Some((&seed_sect, rest)) = sections.split_first() {
            let values = seed_sect[6..]
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let maps = rest
                .iter()
                .map(
                    |section| match section.split(" map:\n").collect_vec().as_slice() {
                        [name, ranges] => (name.to_string(), ranges.parse().unwrap()),
                        _ => panic!("Malformed input"),
                    },
                )
                .collect();

            Ok(Input(values, Almanac { maps }))
        } else {
            Err("Parsing failed")
        }
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            ranges: s.lines().map(|line| line.parse().unwrap()).collect(),
        })
    }
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect_vec().as_slice() {
            [dest_start, src_start, length] => Ok(Range {
                dest_start: dest_start.parse().unwrap(),
                src_start: src_start.parse().unwrap(),
                length: length.parse().unwrap(),
            }),
            _ => Err("Range parsing failed"),
        }
    }
}

// ==== Solution Code ====

impl Almanac {
    fn translate_vec(&self, input: &[usize]) -> Vec<usize> {
        let mut values = input.to_vec();
        for (name, map) in &self.maps {
            values
                .par_iter_mut()
                .progress_with_style(
                    ProgressStyle::with_template("{msg} [{elapsed_precise}] [{bar:.cyan/blue}]")
                        .unwrap()
                        .progress_chars("#>-"),
                )
                .with_message(name.to_string())
                .with_finish(ProgressFinish::AndLeave)
                .for_each(|seed| *seed = map.translate(*seed));
        }
        values
    }
}

impl Map {
    fn translate(&self, input: usize) -> usize {
        for range in &self.ranges {
            if range.contains(input) {
                return range.translate(input);
            }
        }
        return input;
    }
}

impl Range {
    fn contains(&self, input: usize) -> bool {
        self.src_start <= input && input < self.src_start + self.length
    }

    fn translate(&self, input: usize) -> usize {
        if self.contains(input) {
            self.dest_start + (input - self.src_start)
        } else {
            input
        }
    }
}

impl Solution for Day5 {
    fn part1(input: &str) -> Option<usize> {
        let Input(seeds, mut almanac) = input.parse().unwrap();
        almanac.translate_vec(&seeds).iter().copied().min()
    }

    fn part2(input: &str) -> Option<usize> {
        println!("Starting part 2");
        let Input(seeds, mut almanac) = input.parse().unwrap();

        ThreadPoolBuilder::new().num_threads(2).build_global();

        seeds
            // .par_windows(2)
            .iter()
            .tuples()
            .filter_map(|(start, length)| {
                println!("Starting {start} {length}");
                let range = (*start..(start + length));
                almanac
                    .translate_vec(&range.collect_vec())
                    .into_iter()
                    .min()
            })
            .min()
    }
}
