extern crate core;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use colored::Colorize;
use std::env;
use std::io::Error;
use std::process::Command;
use std::time::Duration;

use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;
use crate::day04::day04;
use crate::day05::day05;
use crate::day06::day06;
use crate::day07::day07;
// use crate::day08::day08;
// use crate::day09::day09;
// use crate::day10::day10;
// use crate::day11::day11;
// use crate::day12::day12;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
mod harness;

fn main() {
    let days = [
        day01().f(),
        day02().f(),
        day03().f(),
        day04().f(),
        day05().f(),
        day06().f(),
        day07().f(),
        // day08().f(),
        // day09().f(),
        // day10().f(),
        // day11().f(),
        // day12().f(),
    ];

    let run_one = |id: usize| (days[id - 1].f)();

    let run_all = || {
        let (p1, p2): (Vec<Duration>, Vec<Duration>) = days.iter().map(|d| (d.f)()).unzip();
        let x = p1
            .iter()
            .chain(p2.iter())
            .map(|e| e.as_micros())
            .sum::<u128>();
        let duration = Duration::from_micros(x as u64);

        println!("~~~~~~~~~~~{{ {} }} ~~~~~~~~~~~", "Total".yellow());
        println!(
            "                                 {:>10}",
            format!("{:?}", duration).purple()
        );
        plot(p1, p2).unwrap();
    };

    let run_latest = || run_one(days.len());

    let args = env::args().collect::<Vec<_>>();

    match args.get(1) {
        Some(arg) => {
            if let Ok(id) = arg.parse::<usize>() {
                run_one(id);
            } else {
                match arg.as_str() {
                    "all" => {
                        run_all();
                    }
                    _ => {
                        run_latest();
                    }
                }
            }
        }
        None => {
            run_latest();
        }
    };
}

fn plot(part1: Vec<Duration>, part2: Vec<Duration>) -> Result<(), Error> {
    let convert = |v: Vec<Duration>| {
        v.into_iter()
            .map(|d| d.as_micros())
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("#")
    };

    // Lol just plot with kotlin who's gonna stop me???
    Command::new("C:\\Users\\Marvin\\.gradle\\jdks\\eclipse_adoptium-18-amd64-windows\\jdk-18.0.2.1+1\\bin\\java.exe")
        .arg("-jar")
        .arg("plotter.jar")
        .arg(convert(part1))
        .arg(convert(part2))
        .spawn()?
        .wait()?;

    Ok(())
}
