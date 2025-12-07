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
            part2: part2(input).to_string(),
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

fn part2(input: &str) -> u64 {
    let grid = parse(input);
    let mut results = HashMap::new();

    let (rows, cols) = grid.keys().max().unwrap();

    for row in (0..=*rows).rev() {
        for col in (0..=*cols).rev() {
            if row == *rows {
                results.insert((row, col), 1);
            } else {
                match grid.get(&(row, col)) {
                    None => (),
                    Some(Point::Space) | Some(Point::Beam) => {
                        let prev = results.get(&(row + 1, col)).unwrap();
                        results.insert((row, col), *prev);
                    }
                    Some(Point::Splitter) => {
                        let p1 = results.get(&(row + 1, col - 1)).unwrap();
                        let p2 = results.get(&(row + 1, col + 1)).unwrap();
                        results.insert((row, col), p1 + p2);
                    }
                }
            }
        }
    }

    let beam_point = find_beam(&grid);
    *results.get(&beam_point).unwrap()
}

fn part1(input: &str) -> u64 {
    let grid = parse(input);

    let initial_beam = find_beam(&grid);
    let mut seen = HashSet::new();
    let mut beams = VecDeque::from([initial_beam]);
    let mut splits = 0;

    while !beams.is_empty() {
        let beam_point = beams.pop_front().unwrap();
        if seen.contains(&beam_point) {
            continue;
        }
        seen.insert(beam_point);
        let next_point = (beam_point.0 + 1, beam_point.1);
        match grid.get(&next_point) {
            None => (),
            Some(Point::Beam) => panic!("Unexpected beam"),
            Some(Point::Space) => {
                beams.push_back(next_point);
            }
            Some(Point::Splitter) => {
                splits += 1;
                let (left, right) = split_point(&next_point);
                beams.push_back(left);
                beams.push_back(right);
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
        assert_eq!(solution.part2, "40");
    }
}
