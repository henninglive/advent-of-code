use crate::Problem;

const WIDTH: usize = 99;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Occupied,
    Floor,
}

#[derive(Debug, Clone)]
pub struct Solution(Vec<Cell>);

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            'L' => Cell::Empty,
            '#' => Cell::Occupied,
            '.' => Cell::Floor,
            c => panic!("unexpected character {}", c),
        }
    }
}

impl Solution {
    pub fn init() -> Box<dyn Problem> {
        let board = include_str!("day11.txt")
            .lines()
            .inspect(|line| assert_eq!(line.len(), WIDTH))
            .flat_map(|s| s.chars().map(Cell::from_char))
            .collect::<Vec<_>>();

        Box::new(Solution(board))
    }

    fn height(&self) -> usize {
        self.0.len() / WIDTH
    }

    fn is_occupied(&self, x: isize, y: isize) -> bool {
        false
    }

    fn step(&self, x: isize, y: isize) -> Cell {

        let num_occupied = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y - 1),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
            .iter()
            .map(|e| self.is_occupied(e.0, e.1))
            .count();

        match num_occupied {
            0 =>
        }

        Cell::Floor
    }

    fn asdf(&self) {
        for y in 0..self.height() {
            for x in 0..WIDTH {}
        }
    }
}

impl Problem for Solution {
    fn part1(&self) -> i64 {
        0
    }

    fn part2(&self) -> i64 {
        self.height()
    }
}

#[test]
fn test_part1() {
    let solution = Solution::init();
    assert_eq!(solution.part1(), 0)
}

#[test]
fn test_part2() {
    let solution = Solution::init();
    assert_eq!(solution.part2(), 0)
}

