use crate::harness::Day;
use crate::harness::Part;

pub fn day01() -> Day<i32, i32> {
    Day::new(1, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        0
    }

    fn solve(&self, input: &[String]) -> i32 {
        0
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        0
    }

    fn solve(&self, input: &[String]) -> i32 {
        0
    }
}
