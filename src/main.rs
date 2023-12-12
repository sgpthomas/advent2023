mod day1;
#[allow(unused)]
mod day10;
#[allow(unused)]
mod day11;
#[allow(unused)]
mod day12;
#[allow(unused)]
mod day13;
#[allow(unused)]
mod day14;
#[allow(unused)]
mod day15;
#[allow(unused)]
mod day16;
#[allow(unused)]
mod day17;
#[allow(unused)]
mod day18;
#[allow(unused)]
mod day19;
mod day2;
#[allow(unused)]
mod day20;
#[allow(unused)]
mod day21;
#[allow(unused)]
mod day22;
#[allow(unused)]
mod day23;
#[allow(unused)]
mod day24;
#[allow(unused)]
mod day25;
#[allow(unused)]
mod day3;
mod day4;
#[allow(unused)]
mod day5;
#[allow(unused)]
mod day6;
#[allow(unused)]
mod day7;
#[allow(unused)]
mod day8;
#[allow(unused)]
mod day9;

use std::{fs::File, io::Read, path::PathBuf, time::Instant};

use argh::FromArgs;
use day1::Day1;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day2::Day2;
use day20::Day20;
use day21::Day21;
use day22::Day22;
use day23::Day23;
use day24::Day24;
use day25::Day25;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;

/// Run the solution for a particular day
#[derive(FromArgs)]
struct Cmdline {
    /// which day to use run solutions for
    #[argh(positional)]
    day: String,

    /// use short data for testing
    #[argh(switch)]
    short: bool,
}

trait Solution {
    fn part1(input: &str) -> Option<usize>;
    fn part2(input: &str) -> Option<usize>;
    fn run(input: &str, short: bool) {
        let (part1_sol, part1_time, part2_sol, part2_time) = if short {
            let (part1_input, part2_input) =
                input.split_at(input.find("---").expect("Couldn't find divider"));
            let part2_input = &part2_input[4..];

            let now = Instant::now();
            let part1_sol = Self::part1(part1_input);
            let part1_time = now.elapsed();

            let now = Instant::now();
            let part2_sol = Self::part2(part2_input);
            let part2_time = now.elapsed();

            (part1_sol, part1_time, part2_sol, part2_time)
        } else {
            let now = Instant::now();
            let part1_sol = Self::part1(input);
            let part1_time = now.elapsed();

            let now = Instant::now();
            let part2_sol = Self::part2(input);
            let part2_time = now.elapsed();

            (part1_sol, part1_time, part2_sol, part2_time)
        };

        println!("Solution");
        println!(" Part 1: {part1_sol:?} (took {part1_time:?})");
        println!(" Part 2: {part2_sol:?} (took {part2_time:?})");
    }
}

fn main() {
    let args: Cmdline = argh::from_env();

    // construct file path from command line arguments
    let data_path = if args.short {
        PathBuf::from(format!("inputs/{}-short.txt", args.day))
    } else {
        PathBuf::from(format!("inputs/{}.txt", args.day))
    };

    // make sure that the data exists
    if !data_path.exists() {
        panic!("{data_path:?} does not exist!");
    }

    // actually read data into a string
    let mut file_handle = File::open(data_path).expect("Unable to open file");
    let mut input = String::new();
    file_handle
        .read_to_string(&mut input)
        .expect("Unable to read string");

    // print the input if we are using short data
    if args.short {
        println!("{input}");
    }

    match args.day.as_str() {
        "day1" => Day1::run(&input, args.short),
        "day2" => Day2::run(&input, args.short),
        "day3" => Day3::run(&input, args.short),
        "day4" => Day4::run(&input, args.short),
        "day5" => Day5::run(&input, args.short),
        "day6" => Day6::run(&input, args.short),
        "day7" => Day7::run(&input, args.short),
        "day8" => Day8::run(&input, args.short),
        "day9" => Day9::run(&input, args.short),
        "day10" => Day10::run(&input, args.short),
        "day11" => Day11::run(&input, args.short),
        "day12" => Day12::run(&input, args.short),
        "day13" => Day13::run(&input, args.short),
        "day14" => Day14::run(&input, args.short),
        "day15" => Day15::run(&input, args.short),
        "day16" => Day16::run(&input, args.short),
        "day17" => Day17::run(&input, args.short),
        "day18" => Day18::run(&input, args.short),
        "day19" => Day19::run(&input, args.short),
        "day20" => Day20::run(&input, args.short),
        "day21" => Day21::run(&input, args.short),
        "day22" => Day22::run(&input, args.short),
        "day23" => Day23::run(&input, args.short),
        "day24" => Day24::run(&input, args.short),
        "day25" => Day25::run(&input, args.short),
        _ => panic!("Unknown day"),
    }
}
