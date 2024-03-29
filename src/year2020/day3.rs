//! # Day 3: Toboggan Trajectory
//!
//! With the toboggan login problems resolved, you set off toward the airport.
//! While travel by toboggan might be easy, it's certainly not safe:there's very
//! minimal steering and the area is covered in trees.
//! You'll need to see which angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact integer coordinates in a grid.
//! You make a map (your puzzle input) of the open squares `.` and trees `#` you can see.
//! For example:
//! ```text
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//! ```
//! These aren't the only trees, though; due to something you read about once involving arboreal
//! genetics and biome stability, the same pattern repeats to the right many times:
//! ```text
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........#.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...##....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! You start on the open square `.` in the top-left corner and need to reach the bottom
//! (below the bottom-most row on your map).
//!
//! The toboggan can only follow a few specific slopes (you opted for a cheaper model that
//! prefers rational numbers); start by **counting all the trees** you would encounter for the
//! slope **right 3, down 1**:
//!
//! From your starting position at the top-left, check the position that is right 3 and down 1.
//! Then, check the position that is right 3 and down 1 from there, and so on until you go past
//! the bottom of the map.
//!
//! The locations you'd check in the above example are marked here with O where there was an
//! open square and `X` where there was a tree:
//! ```text
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//! In this example, traversing the map using this slope would cause you to encounter `7` trees.
//!
//! Starting at the top-left corner of your map and following a slope of right 3 and down 1,
//! **how many trees would you encounter?**
//!
//! # Part Two
//!
//! Time to check the rest of the slopes - you need to minimize the probability of a sudden
//! arboreal stop, after all.
//!
//! Determine the number of trees you would encounter if, for each of the following slopes,
//! you start at the top-left corner and traverse the map all the way to the bottom:
//!
//! - Right 1, down 1.
//! - Right 3, down 1. (This is the slope you already checked.)
//! - Right 5, down 1.
//! - Right 7, down 1.
//! - Right 1, down 2.
//!
//! In the above example, these slopes would find `2`, `7`, `3`, `4`, and `2` tree(s) respectively;
//! multiplied together, these produce the answer `336`.
//!
//! **What do you get if you multiply together the number of trees encountered on each of the listed
//! slopes?**

use bit_vec::BitVec;

struct Map {
    width: usize,
    height: usize,
    data: BitVec,
}

impl Map {
    fn load() -> Map {
        let mut width: Option<usize> = None;

        let data = include_str!("day3.txt")
            .lines()
            .flat_map(|l| {
                assert_eq!(*width.get_or_insert(l.len()), l.len());
                l.chars().map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("unexpected character {}", c),
                })
            })
            .collect::<BitVec>();

        let width = width.unwrap();
        Map {
            width,
            height: data.len() / width,
            data,
        }
    }

    fn test_slope(&self, step_x: usize, step_y: usize) -> i64 {
        let mut x = 0;
        let mut y = 0;
        let mut num = 0;
        loop {
            x += step_x;
            y += step_y;

            if y >= self.height {
                break;
            }

            num += self.data[y * self.width + x % self.width] as i64
        }
        num
    }
}

pub fn part1() -> i64 {
    let map = Map::load();
    map.test_slope(3, 1)
}

pub fn part2() -> i64 {
    let map = Map::load();
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| map.test_slope(*x, *y))
        .product()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 153)
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 2421944712)
}
