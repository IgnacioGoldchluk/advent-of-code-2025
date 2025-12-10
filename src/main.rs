use argh::FromArgs;
use solutions::solution::Solver;
use std::fs;
use std::time::Instant;

mod solutions;

#[derive(FromArgs)]
/// Executes the given Advent of Code day
struct Args {
    /// the day to run
    #[argh(option)]
    day: u8,
}

fn main() {
    let args: Args = argh::from_env();

    let file_path = format!("inputs/day{}", args.day);
    let file_contents = fs::read_to_string(file_path).unwrap();

    let solver: Box<dyn Solver> = match args.day {
        0 => Box::new(solutions::day0::Day0Solver {}),
        1 => Box::new(solutions::day1::Day1Solver {}),
        2 => Box::new(solutions::day2::Day2Solver {}),
        3 => Box::new(solutions::day3::Day3Solver {}),
        4 => Box::new(solutions::day4::Day4Solver {}),
        5 => Box::new(solutions::day5::Day5Solver {}),
        6 => Box::new(solutions::day6::Day6Solver {}),
        7 => Box::new(solutions::day7::Day7Solver {}),
        8 => Box::new(solutions::day8::Day8Solver {}),
        9 => Box::new(solutions::day9::Day9Solver {}),
        10 => Box::new(solutions::day10::Day10Solver {}),
        _ => todo!("Unreachable"),
    };

    let now = Instant::now();
    let solution = solver.solve(&file_contents);

    let elapsed = now.elapsed();
    println!("Part1: {}, Part2: {}", solution.part1, solution.part2);
    println!("Elapsed: {:.2?}", elapsed);
}
