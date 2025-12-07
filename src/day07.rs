use crate::harness::Day;
use crate::harness::Part;
use std::collections::{HashMap, HashSet};

pub fn day07() -> Day<u64, u64> {
    Day::new(7, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        21
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = Input::from(input);

        let mut beams = HashSet::new();
        beams.insert(input.start);
        let mut splits = 0;

        for i in 1..input.raw.len() - 1 {
            let mut new_beams = HashSet::new();
            for beam in beams {
                if input.raw[i][beam] == '^' {
                    new_beams.insert(beam - 1);
                    new_beams.insert(beam + 1);
                    splits += 1;
                } else {
                    new_beams.insert(beam);
                }
            }
            beams = new_beams;
        }

        splits
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        40
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = Input::from(input);

        let mut beams = HashMap::from([(input.start, 1_u64)]);

        for i in 1..input.raw.len() - 1 {
            let mut new_beams = HashMap::new();
            for (beam, count) in beams {
                if input.raw[i][beam] == '^' {
                    *new_beams.entry(beam - 1).or_insert(0) += count;
                    *new_beams.entry(beam + 1).or_insert(0) += count;
                } else {
                    *new_beams.entry(beam).or_insert(0) += count;
                }
            }
            beams = new_beams;
        }

        beams.values().sum()
    }
}

struct Input {
    start: usize,
    raw: Vec<Vec<char>>,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        Self {
            start: value[0]
                .chars()
                .enumerate()
                .find(|(_, e)| *e == 'S')
                .map(|(i, _)| i)
                .unwrap(),
            raw: value.iter().map(|e| e.chars().collect()).collect(),
        }
    }
}
