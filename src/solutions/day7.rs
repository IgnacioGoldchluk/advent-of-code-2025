use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Error,
};

use crate::solutions::solution;

pub struct Day7Solver;

impl solution::Solver for Day7Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: "".into(),
        }
    }
}

enum Point {
    Space,
    Splitter,
    Beam,
}

type Coord = (usize, usize);
type Grid = HashMap<Coord, Point>;

fn part1(input: &str) -> u64 {
    let grid = parse(input);

    let initial_beam = find_beam(&grid);
    let mut seen = HashSet::new();
    let mut beams = VecDeque::from([initial_beam]);
    let mut splits = 0;

    while !beams.is_empty() {
        let beam_point = beams.pop_front().unwrap();
        seen.insert(beam_point);
        let next_point = (beam_point.0 + 1, beam_point.1);
        match grid.get(&next_point) {
            None => (),
            Some(Point::Beam) => panic!("Unexpected beam"),
            Some(Point::Space) => {
                if !seen.contains(&next_point) {
                    beams.push_back(next_point);
                    seen.insert(next_point);
                }
            }
            Some(Point::Splitter) => {
                splits += 1;
                let (left, right) = split_point(&next_point);
                for p in [left, right] {
                    if !seen.contains(&p) {
                        beams.push_back(p);
                        seen.insert(p);
                    }
                }
            }
        }
    }

    splits
}

fn split_point(point: &Coord) -> (Coord, Coord) {
    ((point.0, point.1 - 1), (point.0, point.1 + 1))
}

fn find_beam(grid: &Grid) -> Coord {
    grid.iter()
        .find_map(|(coord, point)| match point {
            Point::Beam => Some(*coord),
            _ => None,
        })
        .unwrap()
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(move |(col_idx, c)| ((row_idx, col_idx), Point::try_from(c).unwrap()))
        })
        .collect()
}

impl TryFrom<char> for Point {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Point::Space),
            'S' => Ok(Point::Beam),
            '^' => Ok(Point::Splitter),
            _ => Err(Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use crate::solutions::day7::Day7Solver;

    #[test]
    fn test_input() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;
        let solution = Day7Solver.solve(input);
        assert_eq!(solution.part1, "21");
    }
}
