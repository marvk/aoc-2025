use crate::harness::Day;
use crate::harness::Part;
use std::cmp::min;
use std::cmp::{Reverse, max};
use std::ops::{Add, Neg, Sub};

pub fn day09() -> Day<u64, u64> {
    Day::new(9, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        50
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = parse(input);
        let mut result = 0;
        for i1 in 0..input.len() - 1 {
            for i2 in i1 + 1..input.len() {
                result = max(result, input[i1].rect(&input[i2]));
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
        let input = parse(input);
        let mut result = 0;

        let mut vertices = input.clone();
        vertices.push(vertices[0]);
        vertices.push(vertices[1]);
        let vertices = vertices.windows(3).map(Vertex::build).collect::<Vec<_>>();

        let mut lines = input;
        lines.push(lines[0]);
        let mut lines = lines
            .windows(2)
            .map(|e| Line::new(e[0], e[1]))
            .collect::<Vec<_>>();
        lines.sort_by_key(|e| Reverse(e.len));
        let lines = lines;

        for i1 in 0..vertices.len() - 1 {
            for i2 in i1 + 1..vertices.len() {
                let vert1 = &vertices[i1];
                let vert2 = &vertices[i2];

                if vert1.valid.contains(&vert1.center.dir(&vert2.center))
                    && vert2.valid.contains(&vert2.center.dir(&vert1.center))
                {
                    let rect = Rect::new(&vert1.center, &vert2.center);

                    if !lines.iter().any(|line| rect.intersects(line)) {
                        result = max(result, rect.area());
                    }
                }
            }
        }

        result
    }
}

struct Line {
    a: Vec2,
    b: Vec2,
    len: i32,
}

impl Line {
    fn new(a: Vec2, b: Vec2) -> Self {
        Self {
            a,
            b,
            len: (b.x - a.x).abs() + (b.y - a.y).abs(),
        }
    }
}

struct Vertex {
    center: Vec2,
    valid: Vec<Vec2>,
}

impl Vertex {
    fn build(points: &[Vec2]) -> Vertex {
        let center = points[1];
        let from_direction = (center - points[0]).signum();
        let to_direction = (points[2] - center).signum();

        let valid: Vec<Vec2> = match (from_direction, to_direction) {
            (Vec2::NORTH, Vec2::NORTH) => vec![Vec2::NORTH_EAST, Vec2::SOUTH_EAST],
            (Vec2::NORTH, Vec2::EAST) => vec![Vec2::SOUTH_EAST],
            (Vec2::NORTH, Vec2::WEST) => vec![Vec2::NORTH_EAST, Vec2::SOUTH_EAST, Vec2::NORTH_WEST],

            (Vec2::EAST, Vec2::NORTH) => vec![Vec2::NORTH_EAST, Vec2::SOUTH_EAST, Vec2::SOUTH_WEST],
            (Vec2::EAST, Vec2::EAST) => vec![Vec2::SOUTH_EAST, Vec2::SOUTH_WEST],
            (Vec2::EAST, Vec2::SOUTH) => vec![Vec2::SOUTH_WEST],

            (Vec2::SOUTH, Vec2::EAST) => vec![Vec2::SOUTH_EAST, Vec2::SOUTH_WEST, Vec2::NORTH_WEST],
            (Vec2::SOUTH, Vec2::SOUTH) => vec![Vec2::SOUTH_WEST, Vec2::NORTH_WEST],
            (Vec2::SOUTH, Vec2::WEST) => vec![Vec2::NORTH_WEST],

            (Vec2::WEST, Vec2::NORTH) => vec![Vec2::NORTH_EAST],
            (Vec2::WEST, Vec2::SOUTH) => vec![Vec2::NORTH_EAST, Vec2::SOUTH_WEST, Vec2::NORTH_WEST],
            (Vec2::WEST, Vec2::WEST) => vec![Vec2::NORTH_EAST, Vec2::NORTH_WEST],
            _ => panic!(),
        };

        Vertex { center, valid }
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

impl Vec2 {
    pub const NORTH: Self = v(0, -1);
    pub const NORTH_EAST: Self = v(1, -1);
    pub const EAST: Self = v(1, 0);
    pub const SOUTH_EAST: Self = v(1, 1);
    pub const SOUTH: Self = v(0, 1);
    pub const SOUTH_WEST: Self = v(-1, 1);
    pub const WEST: Self = v(-1, 0);
    pub const NORTH_WEST: Self = v(-1, -1);

    fn rect(&self, other: &Self) -> u64 {
        let width = max(self.x, other.x) - min(self.x, other.x) + 1;
        let height = max(self.y, other.y) - min(self.y, other.y) + 1;
        width as u64 * height as u64
    }

    fn signum(&self) -> Self {
        v(self.x.signum(), self.y.signum())
    }

    #[allow(dead_code)]
    fn dir_string(&self) -> &'static str {
        match *self {
            Self::NORTH => "NORTH",
            Self::NORTH_EAST => "NORTH_EAST",
            Self::EAST => "EAST",
            Self::SOUTH_EAST => "SOUTH_EAST",
            Self::SOUTH => "SOUTH",
            Self::SOUTH_WEST => "SOUTH_WEST",
            Self::WEST => "WEST",
            Self::NORTH_WEST => "NORTH_WEST",
            _ => panic!("no michael no that was so not right: {:?}", self),
        }
    }

    fn dir(&self, other: &Self) -> Vec2 {
        (*other - *self).signum()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        v(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        v(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        v(-self.x, -self.y)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Rect {
    min: Vec2,
    max: Vec2,
}

impl Rect {
    fn new(a: &Vec2, b: &Vec2) -> Self {
        Rect {
            min: v(min(a.x, b.x), min(a.y, b.y)),
            max: v(max(a.x, b.x), max(a.y, b.y)),
        }
    }

    fn area(&self) -> u64 {
        (self.max.x - self.min.x + 1) as u64 * (self.max.y - self.min.y + 1) as u64
    }

    fn intersects(&self, line: &Line) -> bool {
        if line.a.x == line.b.x {
            let x = line.a.x;
            let y_low = min(line.a.y, line.b.y);
            let y_high = max(line.a.y, line.b.y);

            (self.min.x < x) && (x < self.max.x) && (self.min.y < y_high) && (y_low < self.max.y)
        } else if line.a.y == line.b.y {
            let y = line.a.y;
            let x_low = min(line.a.x, line.b.x);
            let x_high = max(line.a.x, line.b.x);

            (self.min.y < y) && (y < self.max.y) && (self.min.x < x_high) && (x_low < self.max.x)
        } else {
            panic!();
        }
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
