use crate::solutions::solution;
use std::{cmp, collections::HashSet};

#[derive(Debug)]
struct IdRange {
    low: String,
    high: String,
}
pub struct Day2Solver;

impl solution::Solver for Day2Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input),
            part2: part2(input),
        }
    }
}

fn part2(input: &str) -> String {
    let total: u64 = input
        .split(",")
        .map(to_range)
        .map(|range| invalid_ids_sum_2(range))
        .sum();

    total.to_string()
}

fn part1(input: &str) -> String {
    let repetitions = 2;
    let total: u64 = input
        .split(",")
        .map(to_range)
        .map(|range| invalid_ids_sum(&range, repetitions))
        .sum();

    total.to_string()
}

fn to_range(range: &str) -> IdRange {
    let r: Vec<&str> = range.split("-").take(2).collect();

    match r[..] {
        [low, high] => IdRange {
            low: low.to_string(),
            high: high.to_string(),
        },
        _ => panic!("Unexpected vector length: {}", r.len()),
    }
}

fn invalid_ids_sum_2(range: IdRange) -> u64 {
    (2..=range.high.len())
        .flat_map(|rep| invalid_ids(&range, rep))
        .collect::<HashSet<u64>>()
        .iter()
        .sum()
}

fn invalid_ids(range: &IdRange, repetitions: usize) -> Vec<u64> {
    let high: u64 = range.high.parse().unwrap();
    (first_invalid_id_after(&range.low, repetitions)..)
        .map(|n| generate_invalid_id(n, repetitions))
        .take_while(|n| *n <= high)
        .collect()
}

fn invalid_ids_sum(range: &IdRange, repetitions: usize) -> u64 {
    // Invalid IDs are duplicate sequences of digits
    // "99", "1212", "123123", "6565", etc.
    // The fastest way is to construct the ids starting from the smallest one
    // greater than the lowest range, increment by one and collect until we exceed
    // the highest range
    invalid_ids(range, repetitions).iter().sum()
}

fn first_invalid_id_after(number: &str, repetitions: usize) -> u64 {
    // We need to start with a 1 followed by zeroes. The number
    // of zeroes is (nl / repetitions) - 1, where `nl` is the length of the number. This
    // is because we need repeated patterns twice (hence dividing by repetitions) and
    // one of the digits is reserved for the first `1`
    let nl = number.len();
    let length = if nl.is_multiple_of(repetitions) {
        nl
    } else {
        nl + 1
    };
    let invalid_num: u64 = ("1".to_string() + &"0".repeat(cmp::max(0, (length / repetitions) - 1)))
        .parse()
        .unwrap();

    let start: u64 = number.parse().unwrap();

    (invalid_num..)
        .into_iter()
        .find(|n| generate_invalid_id(*n, repetitions) >= start)
        .unwrap()
}

fn generate_invalid_id(number: u64, repetitions: usize) -> u64 {
    number.to_string().repeat(repetitions).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day2::Day2Solver;

    use super::solution::Solver;

    #[test]
    fn test_solve() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let solution = Day2Solver.solve(input);
        assert_eq!(solution.part1, "1227775554");
        assert_eq!(solution.part2, "4174379265");
    }
}
