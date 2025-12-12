use crate::solutions::solution;
use regex::Regex;

pub struct Day12Solver;

impl solution::Solver for Day12Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: "".into(),
        }
    }
}

type Coord = (u32, u32);

#[derive(Debug)]
struct Shape {
    points: Vec<Coord>,
}

#[derive(Debug)]
struct Grid {
    rows: u32,
    cols: u32,
    requirements: Vec<u32>,
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        let points = value
            .lines()
            .skip(1)
            .enumerate()
            .flat_map(|(row, l)| {
                l.char_indices()
                    .filter(|(_col, c)| *c == '#')
                    .map(move |(col, _c)| (row as u32, col as u32))
            })
            .collect();

        Self { points }
    }
}

impl Shape {
    fn area(&self) -> u32 {
        self.points.len() as u32
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"(?x)(?P<rows>\d+)x(?P<cols>\d+):\s(?P<reqs>[\d\s]+)").unwrap();

        let caps = re.captures(value).unwrap();

        let reqs: Vec<u32> = caps["reqs"]
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect();

        Self {
            rows: caps["rows"].parse().unwrap(),
            cols: caps["cols"].parse().unwrap(),
            requirements: reqs,
        }
    }
}

impl Grid {
    fn area(&self) -> u32 {
        self.cols * self.rows
    }

    pub fn can_fit(&self, shapes: &[Shape]) -> bool {
        let total_area = self
            .requirements
            .iter()
            .enumerate()
            .map(|(idx, tot)| shapes[idx].area() * tot)
            .sum();

        self.area().gt(&total_area)
    }
}

fn part1(input: &str) -> u32 {
    let values: Vec<&str> = input.split("\n\n").collect();

    let shapes: Vec<Shape> = values[..values.len() - 1]
        .iter()
        .map(|s| Shape::from(*s))
        .collect();

    let grids: Vec<Grid> = values[values.len() - 1]
        .split("\n")
        .map(Grid::from)
        .collect();

    grids.iter().filter(|g| g.can_fit(&shapes)).count() as u32
}
