use crate::harness::Day;
use crate::harness::Part;

pub fn day01() -> Day<i32, i32> {
    Day::new(1, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        3
    }

    fn solve(&self, input: &[String]) -> i32 {
        let vec = parse(input);

        let mut acc = 0;
        let mut pos = 50;

        for x in vec {
            pos = pos + x;
            if pos % 100 == 0 {
                acc += 1;
            }
        }

        acc
    }
}

fn parse(input: &[String]) -> Vec<i32> {
    input
        .iter()
        .filter(|e| !e.is_empty())
        .map(|e: &String| {
            let factor = if e.starts_with("L") { -1 } else { 1 };
            let x = e[1..].parse::<i32>().unwrap();

            x * factor
        })
        .collect()
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        6
    }

    fn solve(&self, input: &[String]) -> i32 {
        let vec = parse(input);

        let mut acc = 0;
        let mut pos = 50;

        for x in vec {
            for _ in 0..x.abs() {
                pos += x.signum();
                if pos % 100 == 0 {
                    acc += 1;
                }
            }
        }

        // for x in vec {
        //     let start_pos = pos;
        //     let temp_pos = pos + x;
        //     pos = (100 + temp_pos) % 100;
        //     if start_pos != 0 && (temp_pos != pos || pos == 0) {
        //         acc += 1;
        //     }
        // }

        acc
    }
}
