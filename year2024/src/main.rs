mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use advent_of_code_core::{menu, Year};

pub static SOLUTIONS: Year = [
    (Some(day1::part1), Some(day1::part2)),
    (Some(day2::part1), Some(day2::part2)),
    (Some(day3::part1), Some(day3::part2)),
    (Some(day4::part1), Some(day4::part2)),
    (Some(day5::part1), Some(day5::part2)),
    (Some(day6::part1), Some(day6::part2)),
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
