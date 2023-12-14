use std::{
    cmp,
    fmt::Display,
    ops::{Add, Neg},
    str::FromStr,
};

use color_print::cprint;
use itertools::{FoldWhile, Itertools};

use crate::Solution;

pub struct Day10;

#[derive(Debug)]
struct InputGrid {
    grid: Vec<Vec<Pipe>>,
    width: usize,
    height: usize,
}

impl FromStr for InputGrid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| line.chars().map(Pipe::from).collect_vec())
            .collect_vec();
        let width = grid[0].len() - 1;
        let height = grid.len() - 1;
        Ok(InputGrid {
            grid,
            width,
            height,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
    Nope,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

impl Add<&Direction> for Pos {
    type Output = Pos;

    fn add(self, rhs: &Direction) -> Self::Output {
        let Pos(x, y) = self;
        match rhs {
            Direction::North => Pos(x, y - 1),
            Direction::East => Pos(x + 1, y),
            Direction::South => Pos(x, y + 1),
            Direction::West => Pos(x - 1, y),
            Direction::Nope => Pos(x, y),
        }
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::Nope => Direction::Nope,
        }
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEastBend,
            'J' => Pipe::NorthWestBend,
            '7' => Pipe::SouthWestBend,
            'F' => Pipe::SouthEastBend,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Unknown char"),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::Vertical => write!(f, "│"),
            Pipe::Horizontal => write!(f, "─"),
            Pipe::NorthEastBend => write!(f, "└"),
            Pipe::NorthWestBend => write!(f, "┘"),
            Pipe::SouthWestBend => write!(f, "┐"),
            Pipe::SouthEastBend => write!(f, "┌"),
            Pipe::Ground => write!(f, "•"),
            Pipe::Start => write!(f, "S"),
        }
    }
}

impl Pipe {
    fn connection_directions(&self) -> (Direction, Direction) {
        match self {
            Pipe::Vertical => (Direction::North, Direction::South),
            Pipe::Horizontal => (Direction::East, Direction::West),
            Pipe::NorthEastBend => (Direction::North, Direction::East),
            Pipe::NorthWestBend => (Direction::North, Direction::West),
            Pipe::SouthWestBend => (Direction::South, Direction::West),
            Pipe::SouthEastBend => (Direction::South, Direction::East),
            Pipe::Ground => (Direction::Nope, Direction::Nope),
            Pipe::Start => (Direction::Nope, Direction::Nope),
        }
    }
}

impl InputGrid {
    fn start_position(&self) -> Pos {
        for x in 0..=self.width {
            for y in 0..=self.height {
                if self.grid[y][x] == Pipe::Start {
                    return Pos(x, y);
                }
            }
        }
        panic!("Didn't find any start position");
    }

    /// Given the position of a pipe, search neighbors for connecting pipes.
    /// If there is one that exists, return that pipe.
    /// There is almost certainly a smarter way of doing this. But I've already commited lol.
    fn next_position(&self, Pos(x, y): Pos, from: Direction) -> Option<Direction> {
        match (&self.grid[y][x], from) {
            (Pipe::Vertical, Direction::North)
                if self.is_connected_to(Pos(x, y), Direction::South) =>
            {
                Some(Direction::South)
            }
            (Pipe::Vertical, Direction::South)
                if self.is_connected_to(Pos(x, y), Direction::North) =>
            {
                Some(Direction::North)
            }
            (Pipe::Vertical, _) => None,

            (Pipe::Horizontal, Direction::West)
                if self.is_connected_to(Pos(x, y), Direction::East) =>
            {
                Some(Direction::East)
            }
            (Pipe::Horizontal, Direction::East)
                if self.is_connected_to(Pos(x, y), Direction::West) =>
            {
                Some(Direction::West)
            }
            (Pipe::Horizontal, _) => None,

            (Pipe::NorthEastBend, Direction::North)
                if self.is_connected_to(Pos(x, y), Direction::East) =>
            {
                Some(Direction::East)
            }
            (Pipe::NorthEastBend, Direction::East)
                if self.is_connected_to(Pos(x, y), Direction::North) =>
            {
                Some(Direction::North)
            }
            (Pipe::NorthEastBend, _) => None,

            (Pipe::NorthWestBend, Direction::West)
                if self.is_connected_to(Pos(x, y), Direction::North) =>
            {
                Some(Direction::North)
            }
            (Pipe::NorthWestBend, Direction::North)
                if self.is_connected_to(Pos(x, y), Direction::West) =>
            {
                Some(Direction::West)
            }
            (Pipe::NorthWestBend, _) => None,

            (Pipe::SouthWestBend, Direction::South)
                if self.is_connected_to(Pos(x, y), Direction::West) =>
            {
                Some(Direction::West)
            }
            (Pipe::SouthWestBend, Direction::West)
                if self.is_connected_to(Pos(x, y), Direction::South) =>
            {
                Some(Direction::South)
            }
            (Pipe::SouthWestBend, _) => None,

            (Pipe::SouthEastBend, Direction::South)
                if self.is_connected_to(Pos(x, y), Direction::East) =>
            {
                Some(Direction::East)
            }
            (Pipe::SouthEastBend, Direction::East)
                if self.is_connected_to(Pos(x, y), Direction::South) =>
            {
                Some(Direction::South)
            }
            (Pipe::SouthEastBend, _) => None,

            (Pipe::Ground, _) => None,

            (Pipe::Start, _) if self.is_connected_to(Pos(x, y), Direction::North) => {
                Some(Direction::North)
            }
            (Pipe::Start, _) if self.is_connected_to(Pos(x, y), Direction::East) => {
                Some(Direction::East)
            }
            (Pipe::Start, _) if self.is_connected_to(Pos(x, y), Direction::South) => {
                Some(Direction::South)
            }
            (Pipe::Start, _) if self.is_connected_to(Pos(x, y), Direction::West) => {
                Some(Direction::West)
            }
            (Pipe::Start, _) => None,
        }
    }

    fn is_connected_to(&self, Pos(x, y): Pos, dir: Direction) -> bool {
        match (dir, x, y) {
            // there is nothing north of the top
            (Direction::North, _, 0) => false,
            // nothing east of the right hand side
            (Direction::East, x, _) if x == self.width => false,
            // nothing south of the bottom
            (Direction::South, _, y) if y == self.height => false,
            // nothing west of the left hand side
            (Direction::West, 0, _) => false,

            (Direction::North, _, _) => {
                let (d0, d1) = self.grid[y - 1][x].connection_directions();
                d0 == Direction::South || d1 == Direction::South
            }
            (Direction::East, _, _) => {
                let (d0, d1) = self.grid[y][x + 1].connection_directions();
                d0 == Direction::West || d1 == Direction::West
            }
            (Direction::South, _, _) => {
                let (d0, d1) = self.grid[y + 1][x].connection_directions();
                d0 == Direction::North || d1 == Direction::North
            }
            (Direction::West, _, _) => {
                let (d0, d1) = self.grid[y][x - 1].connection_directions();
                d0 == Direction::East || d1 == Direction::East
            }
            (Direction::Nope, _, _) => false,
        }
    }

    fn find_path(&self) -> Vec<Pos> {
        let (_, _, path) = (0..)
            .fold_while(
                (self.start_position(), Direction::Nope, vec![]),
                |(pos, dir, mut hist), _| {
                    if let Some(next_dir) = self.next_position(pos, -dir) {
                        hist.push(pos + &next_dir);
                        FoldWhile::Continue((pos + &next_dir, next_dir, hist))
                    } else {
                        FoldWhile::Done((pos, dir, hist))
                    }
                },
            )
            .into_inner();
        path
    }

    fn winding_number(&self, path: &[Pos], dir: Direction) -> i64 {
        let (_, winding) = path
            .iter()
            .map(|Pos(x, y)| &self.grid[*y][*x])
            .filter(|&pipe| pipe != &Pipe::Vertical && pipe != &Pipe::Horizontal)
            .fold((dir, 0), |(dir, winding), pipe| match (pipe, -dir) {
                (Pipe::Vertical, _)
                | (Pipe::Horizontal, _)
                | (Pipe::Ground, _)
                | (Pipe::Start, _) => unreachable!("wrong pipe"),

                (Pipe::NorthEastBend, Direction::North) => (Direction::East, winding - 1),
                (Pipe::NorthEastBend, Direction::East) => (Direction::North, winding + 1),
                (Pipe::NorthEastBend, dir) => unreachable!("north east {dir:?}"),

                (Pipe::NorthWestBend, Direction::West) => (Direction::North, winding - 1),
                (Pipe::NorthWestBend, Direction::North) => (Direction::West, winding + 1),
                (Pipe::NorthWestBend, _) => unreachable!("north west"),

                (Pipe::SouthWestBend, Direction::South) => (Direction::West, winding - 1),
                (Pipe::SouthWestBend, Direction::West) => (Direction::South, winding + 1),
                (Pipe::SouthWestBend, _) => unreachable!("south west"),

                (Pipe::SouthEastBend, Direction::East) => (Direction::South, winding - 1),
                (Pipe::SouthEastBend, Direction::South) => (Direction::East, winding + 1),
                (Pipe::SouthEastBend, _) => unreachable!("south east"),
            });
        if winding > 0 {
            1
        } else {
            -1
        }
    }

    fn display_with_path(&self, path: &[Pos], outside: &[Pos]) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, el) in row.iter().enumerate() {
                if Pos(x, y) == path[0] {
                    cprint!("<red>{el}</>");
                } else if path.contains(&Pos(x, y)) {
                    cprint!("<green>{el}</>");
                } else if outside.contains(&Pos(x, y)) {
                    cprint!("<blue>{el}</>");
                } else {
                    print!("{el}");
                }
            }
            println!();
        }
    }
}

fn uadd(a: usize, b: i64) -> usize {
    ((a as i64) + b) as usize
}

impl Solution for Day10 {
    fn part1(input: &str) -> Option<i64> {
        let grid: InputGrid = input.parse().unwrap();

        let path = grid.find_path();
        path.iter()
            .enumerate()
            .map(|(i, pos)| cmp::min(i + 1, path.len() - i))
            .max()
            .map(|inner| inner as i64)
    }

    fn part2(input: &str) -> Option<i64> {
        let grid: InputGrid = input.parse().unwrap();
        let path: Vec<Pos> = grid.find_path();
        let dir = grid
            .next_position(grid.start_position(), Direction::Nope)
            .unwrap();
        // let path = grid.find_path().into_iter().rev().collect_vec();
        // let dir = Direction::South;

        let winding_number = grid.winding_number(&path, dir);

        let (dir, mut inside) = path
            .iter()
            .map(|Pos(x, y)| (Pos(*x, *y), &grid.grid[*y][*x]))
            .fold((dir, vec![]), |(dir, mut hist), (Pos(x, y), pipe)| {
                let exit_dir = match (pipe, -dir) {
                    (Pipe::Vertical, Direction::North) => Direction::South,
                    (Pipe::Vertical, Direction::South) => Direction::North,
                    (Pipe::Vertical, _) => unreachable!(),

                    (Pipe::Horizontal, Direction::East) => Direction::West,
                    (Pipe::Horizontal, Direction::West) => Direction::East,
                    (Pipe::Horizontal, _) => unreachable!(),

                    (Pipe::NorthEastBend, Direction::North) => {
                        hist.push(Pos(uadd(x, -winding_number), uadd(y, 0)));
                        Direction::East
                    }
                    (Pipe::NorthEastBend, Direction::East) => {
                        hist.push(Pos(uadd(x, 0), uadd(y, -winding_number)));
                        Direction::North
                    }
                    (Pipe::NorthEastBend, dir) => unreachable!("north east {dir:?}"),

                    (Pipe::NorthWestBend, Direction::North) => {
                        hist.push(Pos(uadd(x, -winding_number), uadd(y, 0)));
                        Direction::West
                    }
                    (Pipe::NorthWestBend, Direction::West) => {
                        hist.push(Pos(uadd(x, 0), uadd(y, winding_number)));
                        Direction::North
                    }
                    (Pipe::NorthWestBend, dir) => unreachable!("north west {dir:?}"),

                    (Pipe::SouthWestBend, Direction::South) => {
                        hist.push(Pos(uadd(x, winding_number), uadd(y, 0)));
                        Direction::West
                    }
                    (Pipe::SouthWestBend, Direction::West) => {
                        hist.push(Pos(uadd(x, 0), uadd(y, winding_number)));
                        Direction::South
                    }
                    (Pipe::SouthWestBend, dir) => unreachable!("south west {dir:?}"),

                    (Pipe::SouthEastBend, Direction::East) => {
                        hist.push(Pos(uadd(x, 0), uadd(y, -winding_number)));
                        Direction::South
                    }
                    (Pipe::SouthEastBend, Direction::South) => {
                        hist.push(Pos(uadd(x, winding_number), uadd(y, 0)));
                        Direction::East
                    }
                    (Pipe::SouthEastBend, dir) => unreachable!("south east {dir:?}"),

                    (Pipe::Ground, _) | (Pipe::Start, _) => unreachable!(),
                };
                match exit_dir {
                    Direction::North => hist.push(Pos(uadd(x, winding_number), uadd(y, 0))),
                    Direction::South => hist.push(Pos(uadd(x, -winding_number), uadd(y, 0))),
                    Direction::East => hist.push(Pos(uadd(x, 0), uadd(y, winding_number))),
                    Direction::West => hist.push(Pos(uadd(x, 0), uadd(y, -winding_number))),
                    _ => (),
                }
                (exit_dir, hist)
            });
        inside = inside
            .into_iter()
            .filter(|Pos(x, y)| x <= &grid.width && y <= &grid.height)
            .filter(|pos| pos != &grid.start_position())
            .filter(|pos| !path.contains(pos))
            .unique()
            .collect_vec();

        // flood the inside points that we have
        let mut worklist = inside.clone();
        while let Some(item) = worklist.pop() {
            // check if the four surrounding points should be included, if they haven't been already
            let Pos(x, y) = item;
            let candidates = [
                Pos(x, y.saturating_sub(1)),
                Pos(x, cmp::min(y + 1, grid.height)),
                Pos(cmp::min(x + 1, grid.width), y),
                Pos(x.saturating_sub(1), y),
            ];
            for c in candidates {
                if !path.contains(&c) && !inside.contains(&c) {
                    inside.push(c);
                    worklist.push(c);
                }
            }
        }

        grid.display_with_path(&path, &inside);
        Some(inside.len() as i64)
    }
}
