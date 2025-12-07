use crate::harness::Day;
use crate::harness::Part;
use std::ops::RangeInclusive;

pub fn day05() -> Day<u64, u64> {
    Day::new(5, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        3
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = Input::from(input);

        let i = input
            .inventory
            .iter()
            .filter(|e| input.ranges.iter().any(|range| range.contains(e)))
            .count();

        i as u64
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        14
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = Input::from(input);

        let mut ranges = input.ranges.into_iter().map(Some).collect::<Vec<_>>();

        let mut size_before = ranges.iter().filter(|e| e.is_some()).count();
        let mut size = size_before;
        loop {
            for i1 in 0..(ranges.len() - 1) {
                if ranges[i1].is_none() {
                    continue;
                }
                for i2 in i1 + 1..ranges.len() {
                    if ranges[i2].is_none() {
                        continue;
                    }
                    let r1 = ranges[i1].as_ref().unwrap();
                    let r2 = ranges[i2].as_ref().unwrap();

                    let contains_start = r1.contains(r2.start());
                    let contains_end = r1.contains(r2.end());

                    let contains_start_1 = r2.contains(r1.start());
                    let contains_end_1 = r2.contains(r1.end());

                    if contains_start && contains_end {
                        size -= 1;
                        ranges[i2] = None;
                    } else if contains_start_1 && contains_end_1 {
                        size -= 1;
                        ranges[i1] = None;
                        ranges.swap(i1, i2);
                    } else if contains_start {
                        size -= 1;
                        ranges[i1] = Some(RangeInclusive::new(*r1.start(), *r2.end()));
                        ranges[i2] = None;
                    } else if contains_end {
                        size -= 1;
                        ranges[i1] = Some(RangeInclusive::new(*r2.start(), *r1.end()));
                        ranges[i2] = None;
                    }
                }
            }

            if size_before == size {
                break;
            }
            size_before = size;
        }

        ranges
            .iter()
            .flatten()
            .map(|e| e.end() - e.start() + 1)
            .sum()
    }
}

struct Input {
    ranges: Vec<RangeInclusive<u64>>,
    inventory: Vec<u64>,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let vec = value.split(|e| e.is_empty()).collect::<Vec<_>>();

        let ranges_raw = vec[0];
        let inventory_raw = vec[1];

        let ranges = ranges_raw
            .iter()
            .map(|e| {
                let range = e
                    .split("-")
                    .map(|e| e.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();

                RangeInclusive::new(range[0], range[1])
            })
            .collect();

        let inventory = inventory_raw.iter().map(|e| e.parse().unwrap()).collect();

        Self { ranges, inventory }
    }
}
