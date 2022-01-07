//! # Day 5: Binary Boarding
//!
//! You board your plane only to discover a new problem: you dropped your boarding pass!
//! You aren't sure which seat is yours, and all of the flight attendants are busy with the
//! flood of people that suddenly made it through passport control.
//!
//! You write a quick program to use your phone's camera to scan all of the nearby boarding passes
//! (your puzzle input); perhaps you can find your seat through process of elimination.
//!
//! Instead of [zones or groups](https://www.youtube.com/watch?v=oAHbLRjF0vo),
//! this airline uses binary space partitioning to seat people.
//! A seat might be specified like `FBFBBFFRLR`,
//! where `F` means "front", `B` means "back", `L` means "left", and `R` means "right".
//!
//! The first `7` characters will either be `F` or `B`; these specify exactly one of the
//! **128 rows** on the plane (numbered `0` through `127`). Each letter tells you which half of
//! a region the given seat is in. Start with the whole list of rows; the first letter indicates
//! whether the seat is in the front (`0` through `63`) or the back (`64` through `127`).
//! The next letter indicates which half of that region the seat is in,
//! and so on until you're left with exactly one row.
//!
//! For example, consider just the first seven characters of `FBFBBFFRLR`:
//!
//! - Start by considering the whole range, rows `0` through `127`.
//! - F means to take the **lower half**, keeping rows `0` through `63`.
//! - B means to take the **upper half**, keeping rows `32` through `63`.
//! - F means to take the **lower half**, keeping rows `32` through `47`.
//! - B means to take the **upper half**, keeping rows `40` through `47`.
//! - B keeps rows `44` through `47`.
//! - F keeps rows `44` through `45`.
//! - The final `F` keeps the lower of the two, **row `44`**.
//!
//! The last three characters will be either `L` or `R`; these specify exactly one of the
//! **8 columns** of seats on the plane (numbered `0` through `7`).
//! The same process as above proceeds again, this time with only three steps.
//! `L` means to keep the lower half, while `R` means to keep the **upper half**.
//!
//! For example, consider just the last 3 characters of `FBFBBFFRLR`:
//!
//! - Start by considering the whole range, columns `0` through `7`.
//! - R means to take the upper half, keeping columns `4` through `7`.
//! - L means to take the lower half, keeping columns `4` through `5`.
//! - The final R keeps the upper of the two, column `5`.
//!
//! So, decoding `FBFBBFFRLR` reveals that it is the seat at **row `44`**, **column `5`**.
//!
//! Every seat also has a unique **seat ID**: multiply the row by `8`, then add the column.
//! In this example, the seat has ID `44 * 8 + 5 = 357`.
//!
//! Here are some other boarding passes:
//!
//! - `BFFFBBFRRR`: row `70`, column `7`, seat ID `567`.
//! - `FFFBBBFRRR`: row `14`, column `7`, seat ID `119`.
//! - `BBFFBBFRLL`: row `102`, column `4`, seat ID `820`.
//!
//! As a sanity check, look through your list of boarding passes.
//! What is the highest seat ID on a boarding pass?
//!
//! # Part Two
//!
//! **Ding!** The "fasten seat belt" signs have turned on. Time to find your seat.
//!
//! It's a completely full flight, so your seat should be the only missing boarding pass in
//! your list. However, there's a catch: some of the seats at the very front and back of the
//! plane don't exist on this aircraft, so they'll be missing from your list as well.
//!
//! Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours
//! will be in your list.
//!
//! **What is the ID of your seat?**
//!

use bit_vec::BitVec;

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

fn init() -> Vec<Seat> {
    include_str!("day5.txt")
        .lines()
        .map(|s| Seat::parse(s))
        .collect::<Vec<_>>()
}

pub fn part1() -> i64 {
    init()
        .iter()
        .map(|s| s.id())
        .max()
        .unwrap()
        as i64
}

pub fn part2() -> i64 {
    let mut grid = BitVec::from_elem(8 * 128, false);
    init()
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
    assert_eq!(part1(), 963)
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 592)
}
