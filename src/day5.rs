use crate::Problem;
use bit_vec::BitVec;

pub struct Solution(Vec<Seat>);

struct Seat {
    row: u16,
    column: u16,
}

impl Seat {
    fn parse(s: &str) -> Seat {
        assert_eq!(s.len(), 10);
        let (row_str, column_str) = s.split_at(7);

        let row = row_str
            .chars()
            .map(|c| match c {
                'B' => true,
                'F' => false,
                _ => panic!("unknown character {}", c),
            })
            .rev()
            .enumerate()
            .fold(0u16, |acc, (i, c)| acc | ((c as u16) << i));

        let column = column_str
            .chars()
            .map(|c| match c {
                'R' => true,
                'L' => false,
                _ => panic!("unknown character {}", c),
            })
            .rev()
            .enumerate()
            .fold(0u16, |acc, (i, c)| acc | ((c as u16) << i));

        Seat { row, column }
    }

    fn id(&self) -> i64 {
        self.row as i64 * 8 + self.column as i64
    }
}

impl Solution {
    pub fn init() -> Box<dyn Problem> {
        let seats = include_str!("day5.txt")
            .lines()
            .map(|s| Seat::parse(s))
            .collect::<Vec<_>>();

        Box::new(Solution(seats))
    }
}

impl Problem for Solution {
    fn part1(&self) -> i64 {
        self.0
            .iter()
            .map(|s| s.id())
            .max()
            .unwrap()
            as i64
    }

    fn part2(&self) -> i64 {
        let mut grid = BitVec::from_elem(8 * 128, false);
        self.0
            .iter()
            .for_each(|seat| grid.set(seat.id() as usize, true));

        let exists = |idx: i64| {
            if (0..8 * 128).contains(&idx) {
                grid.get(idx as usize).unwrap()
            } else {
                false
            }
        };

        (0..8 * 128)
            .filter(|&id| !exists(id) && exists(id + 1) && exists(id - 1))
            .next()
            .unwrap()
    }
}

#[test]
fn test_seat_parsing() {
    let test_seat = |str: &str, row: u16, column: u16, id: i64| {
        let seat = Seat::parse(str);
        assert_eq!(seat.row, row);
        assert_eq!(seat.column, column);
        assert_eq!(seat.id(), id)
    };

    test_seat("FBFBBFFRLR", 44, 5, 357);
    test_seat("BFFFBBFRRR", 70, 7, 567);
    test_seat("FFFBBBFRRR", 14, 7, 119);
    test_seat("BBFFBBFRLL", 102, 4, 820);
}

#[test]
fn test_part1() {
    let solution = Solution::init();
    assert_eq!(solution.part1(), 963)
}

#[test]
fn test_part2() {
    let solution = Solution::init();
    assert_eq!(solution.part2(), 592)
}
