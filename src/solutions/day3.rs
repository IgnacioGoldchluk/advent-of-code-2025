use crate::solutions::solution;

pub struct Day3Solver;

impl solution::Solver for Day3Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input),
            part2: part2(input),
        }
    }
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|bank| max_joltage(bank, 12))
        .sum::<u64>()
        .to_string()
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|bank| max_joltage(bank, 2))
        .sum::<u64>()
        .to_string()
}

fn max_joltage(bank: &str, num_batteries: u8) -> u64 {
    match num_batteries {
        0 => 0,
        n => do_max_joltage(bank, n),
    }
}

fn do_max_joltage(bank: &str, num_batteries: u8) -> u64 {
    let mut d = '0';
    let mut idx = 0;

    for (i, c) in bank[0..(bank.len() - (num_batteries as usize - 1))]
        .chars()
        .enumerate()
    {
        if c == '9' {
            d = c;
            idx = i;
            break;
        }
        if c > d {
            d = c;
            idx = i;
        }
    }

    d.to_digit(10).unwrap() as u64 * 10u64.pow(num_batteries as u32 - 1)
        + max_joltage(&bank[idx + 1..], num_batteries - 1)
}

#[cfg(test)]
mod tests {
    use crate::solutions::day3::Day3Solver;

    use super::solution::Solver;

    #[test]
    fn test_solve() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

        let solution = Day3Solver.solve(input);
        assert_eq!(solution.part1, "357");
        assert_eq!(solution.part2, "3121910778619");
    }
}
