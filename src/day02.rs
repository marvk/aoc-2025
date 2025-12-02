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
        solve(parse(input), 2)
    }
}

pub struct Part2;

impl Part<i64> for Part2 {
    fn expect_test(&self) -> i64 {
        4174379265
    }

    fn solve(&self, input: &[String]) -> i64 {
        solve(parse(input), usize::MAX)
    }
}

fn parse(input: &[String]) -> impl Iterator<Item = i64> {
    input[0].split(',').flat_map(|e| {
        let divider = e.find('-').unwrap();
        let from = e[0..divider].parse::<i64>().unwrap();
        let to = e[divider + 1..].parse::<i64>().unwrap();
        from..=to
    })
}

fn solve<I>(iter: I, max_repeats: usize) -> i64
where
    I: Iterator<Item = i64>,
{
    let mut sum = 0;

    for num in iter {
        let num_len = (num.checked_ilog10().unwrap_or(0) + 1) as usize;
        'outer: for n_digits in (1..=(num_len / 2))
            .filter(|e| num_len.is_multiple_of(*e))
            .filter(|e| num_len / e <= max_repeats)
        {
            let divisor = 10_i64.pow(n_digits as u32);

            let mut current = num;
            let quotient = current % divisor;

            while current != 0 {
                if current % divisor != quotient {
                    continue 'outer;
                }

                current /= divisor;
            }

            sum += num;
            break;
        }
    }

    sum
}
