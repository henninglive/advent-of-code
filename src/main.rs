mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day12;

pub trait Problem {
    fn part1(&self) -> i64;
    fn part2(&self) -> i64;
}

macro_rules! solutions {
    ( $( $name:ident ),* ) => {
        fn problems() -> std::collections::BTreeMap<usize, fn() -> Box<dyn Problem>> {
            let mut problems = std::collections::BTreeMap::new();
            let key = |name: &str| name.trim_start_matches("day").parse::<usize>().unwrap();
            $(
                problems.insert(
                    key(stringify!($name)),
                    $name::Solution::init as fn() -> Box<dyn Problem>,
                );
            )*
            problems
        }
    };
}

solutions!(
    day1,
    day2,
    day3,
    day4,
    day5,
    day6,
    day12
);

fn main() {
    let problems = problems();
    match std::env::args().nth(1) {
        Some(ref s) if s == "--help" || s == "-h" => {
            print!("\
Advent of Code is a [website](https://adventofcode.com) with Advent Calendar\n\
of small programming puzzles for a variety of skill sets and skill levels that\n\
can be solved in any programming language.\n\
\n\
This program contains solutions to the Advent of Code 2020 puzzles, solved using\n\
the [Rust programing language](https://www.rust-lang.org/en-US/).\n\
\n\
Usage: advent-of-code [day]\n")
        }
        Some(s) => {
            match s.parse::<usize>().ok() {
                Some(i) => {
                    match problems.get(&i) {
                        Some(init) => {
                            let p = init();
                            println!("Day {}: part 1: {}, part 2: {}", i, p.part1(), p.part2());
                        }
                        None => println!("No solution available for day {}", i),
                    }
                }
                None => println!("Usage: advent-of-code [day]"),
            }
        }
        None => {
            for (i, init) in problems {
                let p = init();
                println!("Day {}: part 1: {}, part 2: {}", i, p.part1(), p.part2());
            }
        }
    }
}
