use itertools::iproduct;
use crate::Problem;

pub struct Solution(Vec<u16>);

impl Solution {
    pub fn init() -> Box<dyn Problem> {
        let numbers = include_str!("day1.txt").lines()
            .map(|s| s.parse::<u16>().unwrap())
            .collect::<Vec<_>>();

        Box::new(Solution(numbers))
    }
}

impl Problem for Solution {
    fn part1(&self) -> i64 {
        iproduct!(self.0.iter(), self.0.iter())
            .find(|(&i, &j)| i + j == 2020)
            .map(|(&i, &j)| i as i64 * j as i64)
            .unwrap()
    }

    fn part2(&self) -> i64 {
        iproduct!(self.0.iter(), self.0.iter(), self.0.iter())
            .find(|(&i, &j, &l)| i + j + l == 2020)
            .map(|(&i, &j, &l)| i as i64 * j as i64 * l as i64)
            .unwrap()
    }
}

#[test]
fn test_part1() {
    let solution = Solution::init();
    assert_eq!(solution.part1(), 960075)
}

#[test]
fn test_part2() {
    let solution = Solution::init();
    assert_eq!(solution.part2(), 212900130)
}
