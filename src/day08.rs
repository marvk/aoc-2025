use crate::harness::Day;
use crate::harness::Part;
use std::collections::HashMap;
use std::ops::Add;

pub fn day08() -> Day<u64, u64> {
    Day::new(8, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        40
    }

    fn solve(&self, input: &[String]) -> u64 {
        let n: usize = if input.len() < 500 { 10 } else { 1000 };
        let input = parse(input);

        let mut circuits = (0..input.len()).collect::<Vec<_>>();

        for IdxDist { i1, i2, .. } in all_dist(&input).into_iter().take(n) {
            let min_index = (circuits[i1], circuits[i2]);
            if min_index.0 != min_index.1 {
                for i in 0..circuits.len() {
                    if circuits[i] == min_index.1 {
                        circuits[i] = min_index.0;
                    }
                }
            }
        }

        let mut vec1 = circuits
            .into_iter()
            .fold(HashMap::new(), |mut acc, e| {
                *acc.entry(e).or_insert(0) += 1;
                acc
            })
            .into_values()
            .collect::<Vec<_>>();

        vec1.sort();

        vec1.into_iter().rev().take(3).product::<usize>() as u64
    }
}

struct IdxDist {
    i1: usize,
    i2: usize,
    dist: f64,
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        25272
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = parse(input);

        let mut circuits = (0..input.len()).collect::<Vec<_>>();
        let mut last_connected = (0, 0);

        for IdxDist { i1, i2, .. } in all_dist(&input) {
            let min_index = (circuits[i1], circuits[i2]);

            if min_index.0 != min_index.1 {
                last_connected = (i1, i2);

                for i in 0..circuits.len() {
                    if circuits[i] == min_index.1 {
                        circuits[i] = min_index.0;
                    }
                }
            }
        }

        input[last_connected.0].x as u64 * input[last_connected.1].x as u64
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

fn parse(input: &[String]) -> Vec<Vec3> {
    input
        .iter()
        .filter(|e| !e.is_empty())
        .map(|e| {
            let mut split = e.split(",").map(|e| e.parse::<f64>().unwrap());

            v(
                split.next().unwrap(),
                split.next().unwrap(),
                split.next().unwrap(),
            )
        })
        .collect()
}

fn all_dist(input: &[Vec3]) -> Vec<IdxDist> {
    let mut result = input
        .iter()
        .enumerate()
        .flat_map(|(i1, e1)| {
            input
                .iter()
                .enumerate()
                .filter(move |&(i2, _)| i2 > i1)
                .map(move |(i2, e2)| IdxDist {
                    i1,
                    i2,
                    dist: e1.dist2(e2),
                })
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| (a.dist).partial_cmp(&b.dist).unwrap());
    result
}

impl Vec3 {
    fn dist2(&self, other: &Vec3) -> f64 {
        (self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0)
    }
}

const fn v(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
