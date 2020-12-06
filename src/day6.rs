//! # Day 6: Custom Customs
//!
//! As your flight approaches the regional airport where you'll switch to a much larger plane,
//! [customs declaration forms](https://en.wikipedia.org/wiki/Customs_declaration) are distributed
//! to the passengers.
//!
//!  The form asks a series of 26 yes-or-no questions marked `a` through `z`.
//! All you need to do is identify the questions for which **anyone in your group** answers "yes".
//! Since your group is just you, this doesn't take very long.
//!
//! However, the person sitting next to you seems to be experiencing a language barrier and asks
//! if you can help. For each of the people in their group, you write down the questions for which
//! they answer "yes", one per line. For example:
//!
//! ```text
//! abcx
//! abcy
//! abcz
//! ```
//!
//! In this group, there are **`6`** questions to which anyone answered "yes":
//! `a`, `b`, `c`, `x`, `y`, and `z`. (Duplicate answers to the same question don't count extra;
//! each question counts at most once.)
//!
//! Another group asks for your help, then another, and eventually you've collected answers from
//! every group on the plane (your puzzle input). Each group's answers are separated by a blank line,
//! and within each group, each person's answers are on a single line. For example:
//!
//! ```text
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//! - The first group contains one person who answered "yes" to **`3`** questions:
//! `a`, `b`, and `c`.
//! - The second group contains three people; combined, they answered "yes" to **`3`** questions:
//! `a`, `b`, and `c`.
//! - The third group contains two people; combined, they answered "yes" to **`3`** questions:
//! `a`, `b`, and `c`.
//! - The fourth group contains four people; combined, they answered "yes" to only **`1`** question,
//! `a`.
//! - The last group contains one person who answered "yes" to only **`1`** question, `b`.
//!
//! In this example, the sum of these counts is `3 + 3 + 3 + 1 + 1` = **`11`**.
//!
//! For each group, count the number of questions to which anyone answered "yes".
//! **What is the sum of those counts?**
//!
//! # Part Two
//!
//! As you finish the last group's customs declaration, you notice that you misread one
//! word in the instructions:
//!
//! You don't need to identify the questions to which **anyone** answered "yes";
//! you need to identify the questions to which **everyone** answered "yes"!
//!
//! Using the same example as above:
//!
//! ```text
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! - In the first group, everyone (all 1 person) answered "yes" to **`3`** questions:
//! `a`, `b`, and `c`.
//! - In the second group, there is **no** question to which everyone answered "yes".
//! - In the third group, everyone answered yes to only **`1`** question, `a`.
//! Since some people did not answer "yes" to `b` or `c`, they don't count.
//! - In the fourth group, everyone answered yes to only **`1`** question, `a`.
//! - In the fifth group, everyone (all 1 person) answered "yes" to **`1`** question, `b`.
//!
//! In this example, the sum of these counts is `3 + 0 + 1 + 1 + 1` = **`6`**.
//!
//! For each group, count the number of questions to which **everyone** answered "yes".
//! **What is the sum of those counts?**

use crate::Problem;

pub struct Solution(Vec<(u32, u32)>);

impl Solution {
    pub fn init() -> Box<dyn Problem> {
        let mut list = Vec::new();
        let mut group: (u32, u32) = (0, u32::max_value());
        for line in include_str!("day6.txt").lines() {
            if line.is_empty() && group != (0, u32::max_value()) {
                list.push(group);
                group = (0, u32::max_value());
                continue;
            }

            let person = line.chars()
                .map(|c| {
                    assert!(('a'..='z').contains(&c));
                    c as u32 - 'a' as u32
                })
                .fold(0u32, |acc, i| acc | 1 << i);

            group.0 |= person;
            group.1 &= person;
        }

        if group != (0, u32::max_value()) {
            list.push(group);
        }

        Box::new(Solution(list))
    }
}

impl Problem for Solution {
    fn part1(&self) -> i64 {
        self.0
            .iter()
            .map(|i| i.0.count_ones())
            .sum::<u32>()
            as i64
    }

    fn part2(&self) -> i64 {
        self.0
            .iter()
            .map(|i| i.1.count_ones())
            .sum::<u32>()
            as i64
    }
}

#[test]
fn test_part1() {
    let solution = Solution::init();
    assert_eq!(solution.part1(), 6680)
}

#[test]
fn test_part2() {
    let solution = Solution::init();
    assert_eq!(solution.part2(), 3117)
}
