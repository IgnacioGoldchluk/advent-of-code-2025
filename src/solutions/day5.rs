use crate::solutions::solution;
use std::cmp;

pub struct Day5Solver;

struct Range(u64, u64);

impl solution::Solver for Day5Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: part2(input).to_string(),
        }
    }
}

fn part2(input: &str) -> usize {
    let (mut ranges, _) = parse(input);
    ranges.sort_by(|r1, r2| r1.0.cmp(&r2.0));

    merge_ranges(&ranges)
        .iter()
        .fold(0, |acc, Range(start, end)| acc + (end - start + 1) as usize)
}

fn part1(input: &str) -> usize {
    let (ranges, numbers) = parse(input);
    numbers.iter().filter(|n| in_any_range(n, &ranges)).count()
}

fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    let (mut start, mut end) = (u64::MAX, u64::MIN);
    let mut merged = Vec::new();

    for r in ranges {
        if r.0 <= end {
            end = cmp::max(r.1, end);
        } else {
            if end != 0 {
                merged.push(Range(start, end));
            }
            start = r.0;
            end = r.1;
        }
    }

    merged.push(Range(start, end));
    merged
}

fn in_any_range(num: &u64, ranges: &[Range]) -> bool {
    ranges.iter().any(|r| *num >= r.0 && *num <= r.1)
}

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let ranges_nums: Vec<&str> = input.split("\n\n").take(2).collect();
    let ranges = ranges_nums[0].lines().map(to_range).collect();
    let nums = ranges_nums[1].lines().map(|n| n.parse().unwrap()).collect();

    (ranges, nums)
}

fn to_range(line: &str) -> Range {
    let start_end = line.split("-").take(2).collect::<Vec<&str>>();
    Range(start_end[0].parse().unwrap(), start_end[1].parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use crate::solutions::day5::Day5Solver;

    #[test]
    fn test_input() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

        let solution = Day5Solver.solve(input);
        assert_eq!(solution.part1, "3");
        assert_eq!(solution.part2, "14")
    }
}
