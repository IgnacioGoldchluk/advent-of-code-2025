pub struct Solution {
    pub part1: String,
    pub part2: String,
}

pub trait Solver {
    fn solve(self, input: String) -> Solution;
}
