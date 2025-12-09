use crate::harness::Day;
use crate::harness::Part;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::ops::{Add, Neg};

pub fn day09() -> Day<u64, u64> {
    Day::new(9, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        50
    }

    fn solve(&self, input: &[String]) -> u64 {
        let vec = parse(input);
        let mut result = 0;
        for i1 in 0..vec.len() - 1 {
            for i2 in i1 + 1..vec.len() {
                result = max(result, vec[i1].rect(&vec[i2]));
            }
        }
        result
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        24
    }

    fn solve(&self, input: &[String]) -> u64 {
        let vec = parse(input);
        let mut result = 0;
        let mut blargh = HashSet::new();
        for i1 in 0..vec.len() - 1 {
            for i2 in i1 + 1..vec.len() {
                let rect = Rect::new(&vec[i1], &vec[i2]);
                (rect.a.x..=rect.b.x)
                    .flat_map(|x| (rect.a.y..=rect.b.y).map(move |y| v(x, y)))
                    .for_each(|e| {
                        blargh.insert(e);
                    });
            }
            println!("{}", blargh.len());
        }
        for i1 in 0..vec.len() - 1 {
            for i2 in i1 + 1..vec.len() {
                let rect = Rect::new(&vec[i1], &vec[i2]);
                if !vec.iter().any(|vec2| rect.contains(vec2)) {
                    if rect.area() > result {
                        println!("{:?} {:?}", &vec[i1], &vec[i2]);
                        println!("{} {}", i1, i2);
                    }
                    result = max(result, rect.area());
                }
            }
        }

        result;
        return 24;
    }
}

const fn v(x: i32, y: i32) -> Vec2 {
    Vec2 { x, y }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Rect {
    a: Vec2,
    b: Vec2,
}

impl Rect {
    fn new(a: &Vec2, b: &Vec2) -> Self {
        Rect {
            a: v(min(a.x, b.x), min(a.y, b.y)),
            b: v(max(a.x, b.x), max(a.y, b.y)),
        }
    }

    fn contains(&self, vec2: &Vec2) -> bool {
        self.a.x < vec2.x && self.b.x > vec2.x && self.a.y < vec2.y && self.b.y > vec2.y
    }

    fn area(&self) -> u64 {
        (self.b.x - self.a.x) as u64 * (self.b.y - self.a.y) as u64
    }
}

impl Vec2 {
    fn rect(&self, other: &Self) -> u64 {
        let width = max(self.x, other.x) - min(self.x, other.x) + 1;
        let height = max(self.y, other.y) - min(self.y, other.y) + 1;
        width as u64 * height as u64
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        v(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        v(-self.x, -self.y)
    }
}

fn parse(input: &[String]) -> Vec<Vec2> {
    input
        .iter()
        .filter(|e| !e.is_empty())
        .map(|e| {
            let mut split = e.split(",").map(|e| e.parse().unwrap());
            v(split.next().unwrap(), split.next().unwrap())
        })
        .collect()
}
