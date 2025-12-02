use crate::solutions::solution;
use std::cmp;

struct IdRange {
    low: String,
    high: String,
}
pub struct Day2Solver;

impl solution::Solver for Day2Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input),
            part2: "0".into(),
        }
    }
}

fn part1(input: &str) -> String {
    let total: u64 = input.split(",").map(to_range).map(invalid_ids_sum).sum();

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

fn invalid_ids_sum(range: IdRange) -> u64 {
    // Invalid IDs are duplicate sequences of digits
    // "99", "1212", "123123", "6565", etc.
    // The fastest way is to construct the ids starting from the smallest one
    // greater than the lowest range, increment by one and collect until we exceed
    // the highest range
    let high: u64 = range.high.parse().unwrap();

    (first_invalid_id_after(&range.low)..)
        .map(generate_invalid_id)
        .take_while(|n| *n <= high)
        .collect::<Vec<u64>>()
        .iter()
        .sum()
}

fn first_invalid_id_after(number: &str) -> u64 {
    // We need to start with a 1 followed by zeroes. The number
    // of zeroes is (nl / 2) - 1, where `nl` is the length of the number. This
    // is because we need repeated patterns twice (hence dividing by 2) and
    // one of the digits is reserved for the first `1`
    let nl = number.len();
    let length = if nl.is_multiple_of(2) { nl } else { nl + 1 };
    let invalid_num: u64 = ("1".to_string() + &"0".repeat(cmp::max(0, (length / 2) - 1)))
        .parse()
        .unwrap();

    let start: u64 = number.parse().unwrap();

    (invalid_num..)
        .into_iter()
        .find(|n| generate_invalid_id(*n) >= start)
        .unwrap()
}

fn generate_invalid_id(number: u64) -> u64 {
    // This could also be done by (number + (number*int(log10(number))))
    number.to_string().repeat(2).parse().unwrap()
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
        assert_eq!(solution.part2, "0");
    }
}
