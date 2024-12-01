mod day1;

use advent_of_code_core::{menu, Year};

pub static SOLUTIONS: Year = [
    (Some(day1::part1), Some(day1::part2)),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
    (None, None),
];

fn main() -> Result<(), String> {
    menu(&SOLUTIONS)
}
