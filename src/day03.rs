use crate::harness::Day;
use crate::harness::Part;

pub fn day03() -> Day<u64, u64> {
    Day::new(3, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        357
    }

    fn solve(&self, input: &[String]) -> u64 {
        solve(parse(input), 2)
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        3121910778619
    }

    fn solve(&self, input: &[String]) -> u64 {
        solve(parse(input), 12)
    }
}

fn solve(vec: Vec<Vec<u64>>, depth: usize) -> u64 {
    vec.iter().map(|e| solve_bank(e, depth)).sum()
}

fn solve_bank(bank: &[u64], depth: usize) -> u64 {
    let mut result_joltage = 0;
    let mut min_index = 0;

    for remaining_digits in (1..=depth).rev() {
        let mut best_joltage_digit = 0;
        let mut best_current_index = 0;
        for (current_index, &current_joltage_digit) in bank
            .iter()
            .enumerate()
            .skip(min_index)
            .take(bank.len() - remaining_digits - min_index + 1)
        {
            if current_joltage_digit > best_joltage_digit {
                best_joltage_digit = current_joltage_digit;
                best_current_index = current_index;
            }
        }

        result_joltage = 10 * result_joltage + best_joltage_digit;
        min_index = best_current_index + 1;
    }

    result_joltage
}

fn parse(input: &[String]) -> Vec<Vec<u64>> {
    input
        .iter()
        .filter(|e| !e.is_empty())
        .map(|e| e.chars().map(|e| e.to_digit(10).unwrap() as u64).collect())
        .collect()
}
