use crate::harness::Day;
use crate::harness::Part;
use std::cmp::max;

pub fn day03() -> Day<i32, i32> {
    Day::new(3, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        357
    }

    fn solve(&self, input: &[String]) -> i32 {
        parse(input)
            .iter()
            .map(|e| {
                let mut best = 0;
                for (i1, &e1) in e.iter().enumerate() {
                    for &e2 in &e[i1 + 1..] {
                        best = max(best, e1 * 10 + e2);
                    }
                }
                best
            })
            .sum::<u32>() as i32
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        todo!()
    }

    fn solve(&self, input: &[String]) -> i32 {
        todo!()
    }
}

fn parse(input: &[String]) -> Vec<Vec<u32>> {
    input
        .iter()
        .filter(|e| !e.is_empty())
        .map(|e| e.chars().map(|e| e.to_digit(10).unwrap()).collect())
        .collect()
}
