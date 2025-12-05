use crate::harness::Day;
use crate::harness::Part;
use std::ops::Add;
use std::ops::Neg;

pub fn day04() -> Day<i32, i32> {
    Day::new(4, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        13
    }

    fn solve(&self, input: &[String]) -> i32 {
        let grid = Grid::from(input);

        let mut result = 0;

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let pos = v(x as i32, y as i32);
                if !grid.get(pos) {
                    continue;
                }
                let mut sum = 0;
                for direction in &Vec2::DIRECTIONS {
                    if grid.get(direction.add(pos)) {
                        sum += 1;
                    }
                }
                if sum < 4 {
                    result += 1;
                }
            }
        }

        result
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        43
    }

    fn solve(&self, input: &[String]) -> i32 {
        let mut grid = Grid::from(input);

        let mut previous_result = 0;
        let mut result = 0;

        loop {
            for y in 0..grid.height() {
                for x in 0..grid.width() {
                    let pos = v(x as i32, y as i32);
                    if !grid.get(pos) {
                        continue;
                    }
                    let mut sum = 0;
                    for direction in &Vec2::DIRECTIONS {
                        if grid.get(direction.add(pos)) {
                            sum += 1;
                        }
                    }
                    if sum < 4 {
                        result += 1;
                        grid.unset(pos);
                    }
                }
            }

            if previous_result == result {
                break;
            }

            previous_result = result;
        }

        result
    }
}

struct Grid {
    raw: Vec<Vec<bool>>,
}

impl Grid {
    fn get(&self, pos: Vec2) -> bool {
        self.raw
            .get(pos.y as usize)
            .and_then(|e| e.get(pos.x as usize))
            .copied()
            .unwrap_or(false)
    }

    fn unset(&mut self, pos: Vec2) {
        self.raw[pos.y as usize][pos.x as usize] = false;
    }

    fn width(&self) -> usize {
        self.raw[0].len()
    }

    fn height(&self) -> usize {
        self.raw.len()
    }
}

impl From<&[String]> for Grid {
    fn from(value: &[String]) -> Self {
        let raw = value
            .iter()
            .filter(|e| !e.is_empty())
            .map(|e| e.chars().map(|e| e == '@').collect())
            .collect();

        Self { raw }
    }
}

const fn v(x: i32, y: i32) -> Vec2 {
    Vec2::new(x, y)
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

    pub const DIRECTIONS: [Self; 8] = [
        Self::NORTH,
        Self::NORTH_EAST,
        Self::EAST,
        Self::SOUTH_EAST,
        Self::SOUTH,
        Self::SOUTH_WEST,
        Self::WEST,
        Self::NORTH_WEST,
    ];

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
