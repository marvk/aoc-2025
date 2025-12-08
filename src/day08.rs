use crate::harness::Day;
use crate::harness::Part;
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
        let mut input = parse(input);
        let mut circuits = (0..input.len()).collect::<Vec<_>>();

        for _ in 0..n {
            let mut min_dist = f32::MAX;
            let mut min_index = (0, 0);

            for i1 in 0..(input.len() - 1) {
                if input[i1].is_empty() {
                    continue;
                }

                for i2 in (i1 + 1)..input.len() {
                    if input[i2].is_empty() {
                        continue;
                    }

                    let dist = dist(&input[i1], &input[i2]);

                    if dist < min_dist {
                        // println!("{:?}", input[i1]);
                        // println!("{:?}", input[i2]);
                        // println!("{}", dist);
                        min_index = (i1, i2);
                        min_dist = dist;
                    }
                }
            }

            println!(
                "merging {:?} and {:?}",
                input[min_index.0], input[min_index.1]
            );

            let mut temp = vec![];
            temp.append(&mut input[min_index.1]);
            input[min_index.0].append(&mut temp);
        }

        println!();
        println!();
        println!();
        println!();

        let mut vec1 = input.into_iter().map(|e| e.len()).collect::<Vec<_>>();
        vec1.sort();

        println!("{:?}", vec1);
        vec1.into_iter().rev().take(3).product::<usize>() as u64
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        todo!()
    }

    fn solve(&self, input: &[String]) -> u64 {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

fn parse(input: &[String]) -> Vec<Vec3> {
    input
        .iter()
        .filter(|e| !e.is_empty())
        .map(|e| {
            let mut split = e.split(",").map(|e| e.parse::<i32>().unwrap());

            v(
                split.next().unwrap(),
                split.next().unwrap(),
                split.next().unwrap(),
            )
        })
        .collect()
}

impl Vec3 {
    fn dist(&self, other: &Vec3) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32)
            .sqrt()
    }
}

fn dist(a: &[Vec3], b: &[Vec3]) -> f32 {
    let mut result = f32::MAX;
    for x in a {
        for y in b {
            result = f32::min(result, x.dist(y));
        }
    }
    result
}

const fn v(x: i32, y: i32, z: i32) -> Vec3 {
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
