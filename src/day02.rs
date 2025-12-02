use crate::harness::Day;
use crate::harness::Part;

pub fn day02() -> Day<i64, i64> {
    Day::new(2, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i64> for Part1 {
    fn expect_test(&self) -> i64 {
        1227775554
    }

    fn solve(&self, input: &[String]) -> i64 {
        let vec = parse(input);

        let mut sum = 0;

        for x in vec {
            for i in (1..=(x.len() / 2)).filter(|e| x.len() % e == 0) {
                let repeat = x.len() / i;

                if repeat != 2 {
                    continue;
                }

                let x1 = &x[0..i];

                let string = (0..repeat).map(|_| x1).collect::<Vec<_>>().join("");
                if string == x {
                    sum += x.parse::<i64>().unwrap();
                }
            }
        }

        sum
    }
}

pub struct Part2;

impl Part<i64> for Part2 {
    fn expect_test(&self) -> i64 {
        4174379265
    }

    fn solve(&self, input: &[String]) -> i64 {
        let vec = parse(input);

        let mut sum = 0;

        for x in vec {
            for i in (1..=(x.len() / 2)).filter(|e| x.len() % e == 0) {
                let repeat = x.len() / i;

                if repeat < 2 {
                    continue;
                }

                let x1 = &x[0..i];

                let string = (0..repeat).map(|_| x1).collect::<Vec<_>>().join("");
                if string == x {
                    let i1 = x.parse::<i64>().unwrap();
                    sum += i1;
                    break;
                }
            }
        }

        sum
    }
}

fn parse(input: &[String]) -> Vec<String> {
    input[0]
        .split(',')
        .flat_map(|e| {
            let vec = e
                .split('-')
                .map(|e| e.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (vec[0]..=vec[1])
        })
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
}
