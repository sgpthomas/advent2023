use regex::Regex;

use crate::Solution;

pub struct Day1;

impl Day1 {
    fn parse(input: &str) -> usize {
        match input {
            "0" | "zero" => 0,
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            x => panic!("{x} is not a digit"),
        }
    }

    /// Return the list of matches starting at every index of the string
    fn overlapping_search<'a>(regex: &'a Regex, input: &'a str) -> Vec<&'a str> {
        (0..input.len())
            .into_iter()
            .filter_map(|i| regex.find(&input[i..]).map(|m| m.as_str()))
            .collect()
    }
}

impl Solution for Day1 {
    fn part1(input: &str) -> Option<i64> {
        Some(
            input
                .lines()
                .map(|line| line.matches(char::is_numeric).collect::<Vec<_>>())
                .map(|digits| (digits[0], digits[digits.len() - 1]))
                .map(|(first, last)| format!("{first}{last}").parse::<usize>().unwrap())
                .sum::<usize>() as i64,
        )
    }

    fn part2(input: &str) -> Option<i64> {
        let re = Regex::new("(one|two|three|four|five|six|seven|eight|nine|zero|[0-9])").unwrap();
        Some(
            input
                // for each line
                .lines()
                // gather all regex matches for that line
                .map(|line| Self::overlapping_search(&re, line))
                // pull out the first and last one
                .map(|digits| (digits[0], digits[digits.len() - 1]))
                .map(|(first, last)| (Self::parse(first), Self::parse(last)))
                .map(|(first, last)| first * 10 + last)
                .sum::<usize>() as i64,
        )
    }
}
