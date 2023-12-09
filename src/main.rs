mod day1;
mod day2;

use std::{fs::File, io::Read, path::PathBuf, time::Instant};

use argh::FromArgs;
use day1::Day1;
use day2::Day2;

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
        _ => panic!("Unknown day"),
    }
}
