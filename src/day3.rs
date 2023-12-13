use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::Solution;

pub struct Day3;

#[derive(Debug, Default)]
struct Schematic {
    /// maps coordinates to numbers
    grid: HashMap<(i32, i32), (usize, i32)>,
    /// list of coordinates where symbols exist
    symbols: Vec<(i32, i32, char)>,
}

#[derive(Clone, Debug)]
enum SchematicItem {
    Number(i32, i32),
    Symbol(char),
    Space,
}

impl Schematic {
    fn nums_adjacent(&self, x: i32, y: i32) -> impl Iterator<Item = i32> + '_ {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter_map(move |(xoff, yoff)| self.grid.get(&(x + xoff, y + yoff)))
            .unique_by(|(id, _)| id)
            .map(|(_, val)| *val)
    }
}

impl SchematicItem {
    fn is_symbol_or_space(c: char) -> bool {
        c == '.' || !c.is_digit(10)
    }
}

impl From<char> for SchematicItem {
    fn from(value: char) -> Self {
        if value == '.' {
            SchematicItem::Space
        } else if value.is_numeric() {
            SchematicItem::Number(value.to_digit(10).unwrap() as i32, 1)
        } else {
            SchematicItem::Symbol(value)
        }
    }
}

impl FromStr for Schematic {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut scheme = Schematic::default();
        let mut id = 0;

        for (y, line) in s.lines().enumerate() {
            line.chars().schematic().fold(0, |x, el| match el {
                SchematicItem::Number(num, digits) => {
                    for i in 0..digits {
                        scheme.grid.insert((x + i, y as i32), (id, num));
                    }
                    id += 1;
                    x + digits
                }
                SchematicItem::Symbol(c) => {
                    scheme.symbols.push((x, y as i32, c));
                    x + 1
                }
                SchematicItem::Space => x + 1,
            });
        }

        Ok(scheme)
    }
}

struct SchematicIter<I>
where
    I: Iterator<Item = char>,
{
    iter: I,
    curr_number: Option<i32>,
    curr_digits: i32,
    saved: Option<SchematicItem>,
}

impl<I> Iterator for SchematicIter<I>
where
    I: Iterator<Item = char>,
{
    type Item = SchematicItem;

    fn next(&mut self) -> Option<Self::Item> {
        // if we have a saved item, yield that instead of calling next on our iterator
        if let Some(si) = &self.saved {
            let copy = si.clone();
            self.saved = None;
            Some(copy)
        } else {
            match (self.iter.next(), self.curr_number) {
                // we find a numeric char and we are not currently building a number
                // so we want to start a new number
                (Some(c), None) if c.is_numeric() => {
                    self.curr_number = Some(c.to_digit(10).unwrap() as i32);
                    self.curr_digits = 1;
                    self.next()
                }
                // we are building a number and we reach a '.' Return the number
                (Some(c), Some(num)) if SchematicItem::is_symbol_or_space(c) => {
                    self.curr_number = None;
                    self.saved = Some(SchematicItem::from(c));
                    Some(SchematicItem::Number(num, self.curr_digits))
                }
                (Some(c), Some(num)) if c.is_numeric() => {
                    self.curr_number = Some((num * 10) + c.to_digit(10).unwrap() as i32);
                    self.curr_digits += 1;
                    self.next()
                }
                (Some(c), None) => Some(SchematicItem::from(c)),
                (None, Some(num)) => {
                    self.curr_number = None;
                    Some(SchematicItem::Number(num, self.curr_digits))
                }
                _ => None,
            }
        }
    }
}

trait SchematicIterAdapter: Iterator<Item = char> + Sized {
    fn schematic(self) -> SchematicIter<Self> {
        SchematicIter {
            iter: self,
            curr_number: None,
            curr_digits: 0,
            saved: None,
        }
    }
}

impl<I: Iterator<Item = char>> SchematicIterAdapter for I {}

impl Solution for Day3 {
    fn part1(input: &str) -> Option<i64> {
        let scheme = input.parse::<Schematic>().unwrap();

        Some(
            scheme
                .symbols
                .iter()
                .map(|(x, y, _)| scheme.nums_adjacent(*x, *y))
                .flatten()
                .sum::<i32>() as i64,
        )
    }

    fn part2(input: &str) -> Option<i64> {
        let scheme = input.parse::<Schematic>().unwrap();

        Some(
            scheme
                .symbols
                .iter()
                .filter(|(_, _, c)| c == &'*')
                .map(|(x, y, _)| scheme.nums_adjacent(*x, *y).collect_vec())
                .filter(|vals| vals.len() == 2)
                .map(|vals| vals[0] * vals[1])
                .sum::<i32>() as i64,
        )
    }
}
