use argh::FromArgs;
use solutions::solution::Solver;
use std::fs;

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
        _ => todo!("Unreachable"),
    };

    let solution = solver.solve(&file_contents);

    println!("Part1: {}, Part2: {}", solution.part1, solution.part2);
}
