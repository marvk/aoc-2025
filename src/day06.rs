use crate::harness::Day;
use crate::harness::Part;
use std::cmp::min;

pub fn day06() -> Day<u64, u64> {
    Day::new(6, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        4277556
    }

    fn solve(&self, input: &[String]) -> u64 {
        solve(parse(input))
    }
}

impl Part1 {}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        3263827
    }

    fn solve(&self, input: &[String]) -> u64 {
        solve(parse2(input))
    }
}

fn solve(input: Vec<Operation>) -> u64 {
    input
        .into_iter()
        .map(|Operation { numbers, operator }| match operator {
            Operator::PLUS => numbers.into_iter().sum::<u64>(),
            Operator::TIMES => numbers.into_iter().product::<u64>(),
        })
        .sum()
}

#[derive(Debug)]
enum Operator {
    PLUS,
    TIMES,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "*" => Operator::TIMES,
            "+" => Operator::PLUS,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Operation {
    numbers: Vec<u64>,
    operator: Operator,
}

fn parse(input: &[String]) -> Vec<Operation> {
    let vec1 = input
        .iter()
        .map(|e| e.split_whitespace().collect::<Vec<_>>())
        .filter(|e| !e.is_empty())
        .collect::<Vec<_>>();

    let mut numbers: Vec<Vec<u64>> = vec![vec![]; vec1[0].len()];

    vec1.iter().take(vec1.len() - 1).for_each(|vec| {
        vec.iter()
            .map(|e| e.parse::<u64>().unwrap())
            .enumerate()
            .for_each(|(i2, e)| {
                numbers[i2].push(e);
            });
    });

    numbers
        .into_iter()
        .enumerate()
        .map(|(i, e)| Operation {
            numbers: e,
            operator: vec1.last().unwrap()[i].into(),
        })
        .collect()
}

fn parse2(input: &[String]) -> Vec<Operation> {
    let mut temp = vec![];
    let height = input.len() - 1;
    for col in 0.. {
        let mut col_string = String::new();
        for row in 0..height {
            if let Some(e) = input[row].chars().nth(col) {
                col_string.push(e);
            }
        }
        if col_string.is_empty() {
            break;
        }
        temp.push(col_string);
    }

    temp.split(|e| e.trim().is_empty())
        .map(|vec| Operation {
            numbers: vec
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    let e = if i == 0 {
                        let mut chars = e.chars();
                        chars.next_back();
                        chars.as_str()
                    } else {
                        e
                    };
                    e.trim().parse().unwrap()
                })
                .collect(),
            operator: vec[0].chars().last().unwrap().to_string().as_str().into(),
        })
        .collect()
}
