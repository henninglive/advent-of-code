mod year2020;
mod year2021;

use structopt::StructOpt;

type Solution = (Option<fn() -> i64>, Option<fn() -> i64>);
type Year = [Solution; 24];

static YEARS: [(u16, &'static Year); 2] = [
    (2020, &year2020::SOLUTIONS),
    (2021, &year2021::SOLUTIONS)
];


/// [Advent of Code](https://adventofcode.com) is a website with Advent Calendars
/// with small programming puzzles for a variety of skill sets and skill levels that
/// can be solved in any programming language.
///
/// This program contains solutions to some of the puzzles, solved using the
/// [Rust programing language](https://www.rust-lang.org/en-US/).
///
#[derive(StructOpt, Debug)]
#[structopt(name = "advent-of-code")]
struct Opt {
    /// Year
    #[structopt()]
    year: Option<u16>,

    /// Day of Advent
    #[structopt()]
    day: Option<u16>,
}

fn print_day(day: u16, solution: Solution) {
    match solution {
        (Some(part1), Some(part2)) => println!("Day {}: part1: {}, part2: {}", day, part1(), part2()),
        (Some(part1), None) => println!("Day {}: part1: {}", day, part1()),
        (None, Some(part2)) => println!("Day {}: part2: {}", day, part2()),
        _ => {}
    }
}

fn main() {
    let opt: Opt = Opt::from_args();
    for (y, days) in YEARS {
        if opt.year.is_none() || opt.year == Some(y) {
            println!("\nAdvent of Code {}:", y);
            match opt.day {
                Some(d) if d < 24 => print_day(d + 1, days[d as usize]),
                _ => (0..24).for_each(|d| print_day(d + 1, days[d as usize]))
            }
        }
    }
}
