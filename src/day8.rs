use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::Solution;

pub struct Day8;

#[derive(Debug)]
struct Input(Vec<Inst>, HashMap<String, Node>);

#[derive(Debug)]
enum Inst {
    L,
    R,
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

// == Parsing ==
impl FromStr for Input {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split("\n\n")
            .tuples()
            .map(|(insts_str, nodes_str)| {
                Input(
                    insts_str.chars().map(|c| c.into()).collect_vec(),
                    nodes_str
                        .lines()
                        .map(|line| line.parse().unwrap())
                        .map(|node: Node| (node.name.to_string(), node))
                        .collect(),
                )
            })
            .next()
            .ok_or("Failed parsing")
    }
}

impl From<char> for Inst {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::L,
            'R' => Self::R,
            _ => panic!("Unknown direction"),
        }
    }
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.replace(" = (", ", ")
            .replace(")", "")
            .split(", ")
            .tuples()
            .map(|(name, left, right)| Node {
                name: name.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            })
            .next()
            .ok_or("Node parsing failed")
    }
}

impl Node {
    fn step<'a>(&self, map: &'a HashMap<String, Node>, inst: &Inst) -> &'a Node {
        match inst {
            Inst::L => &map[&self.left],
            Inst::R => &map[&self.right],
        }
    }

    fn frequency(&self, map: &HashMap<String, Node>, insts: &[Inst]) -> usize {
        let mut freq = 0;
        let mut node_ref = self;
        while !node_ref.name.ends_with("Z") {
            freq += 1;
            for inst in insts {
                node_ref = node_ref.step(map, inst);
            }
        }
        freq
    }
}

impl Solution for Day8 {
    fn part1(input: &str) -> Option<usize> {
        let Input(insts, nodes) = input.parse().unwrap();
        let mut node = &nodes["AAA"];
        let mut insts_iter = insts.iter().cycle();
        let mut count = 0;
        while node.name != "ZZZ" {
            count += 1;
            let inst = insts_iter.next().unwrap();
            node = node.step(&nodes, inst);
        }
        Some(count)
    }

    fn part2(input: &str) -> Option<usize> {
        let Input(insts, map) = input.parse().unwrap();

        let mut insts_iter = insts.iter().cycle();

        // this works by finding the frequency of each node that ends with A
        // seperately. The freqency is the number of full insts cycles that
        // it takes to hit a node that ends with Z. all of the nodes will
        // line up at the lowest common multiple of all the frequencies.
        Some(
            map.keys()
                .filter(|x| x.ends_with("A"))
                .map(|key| &map[key])
                .sorted_by(|n1, n2| n1.name.cmp(&n2.name))
                .map(|node| node.frequency(&map, &insts))
                .product::<usize>()
                * insts.len(),
        )
    }
}
