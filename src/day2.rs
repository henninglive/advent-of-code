use crate::Problem;

pub struct Solution(Vec<Line>);

struct Line(usize, usize, char, &'static str);

impl Line {
    fn scan(line: &'static str) -> Line {
        let parts = line
            .split(&['-', ' ', ':'][..])
            .filter(|s| s.len() > 0)
            .collect::<Vec<_>>();

        assert_eq!(parts.len(), 4);
        let min = parts[0].parse::<usize>().unwrap();
        let max = parts[1].parse::<usize>().unwrap();
        assert!(min <= max);

        assert_eq!(parts[2].len(), 1);
        let char = parts[2].chars().next().unwrap();
        let password = parts[3];
        Line(min, max, char, password)
    }
}

impl Solution {
    pub fn init() -> Box<dyn Problem> {
        let lines = include_str!("day2.txt")
            .lines()
            .map(|s| Line::scan(s))
            .collect::<Vec<_>>();

        Box::new(Solution(lines))
    }
}

impl super::Problem for Solution {
    fn part1(&self) -> i64 {
        self.0
            .iter()
            .filter(|line| (line.0..=line.1).contains(
                &line.3.chars().filter(|c| *c == line.2).count()
            ))
            .count()
            as i64
    }

    fn part2(&self) -> i64 {
        self.0
            .iter()
            .filter(|line| {
                let a = line.3.chars().nth(line.0 - 1).unwrap();
                let b = line.3.chars().nth(line.1 - 1).unwrap();
                (a == line.2) ^ (b == line.2)
            })
            .count()
            as i64
    }
}

#[test]
#[cfg(test)]
fn test_part1() {
    let solution = Solution::init();
    assert_eq!(solution.part1(), 603)
}

#[test]
#[cfg(test)]
fn test_part2() {
    let solution = Solution::init();
    assert_eq!(solution.part2(), 404)
}


