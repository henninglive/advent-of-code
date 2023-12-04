mod year2020;
mod year2021;
mod year2022;
mod year2023;

use clap::Parser;

type Solution = (Option<fn() -> i64>, Option<fn() -> i64>);
type Year = [Solution; 24];

static YEARS: [(u16, &'static Year); 4] = [
    (2020, &year2020::SOLUTIONS),
    (2021, &year2021::SOLUTIONS),
    (2022, &year2022::SOLUTIONS),
    (2023, &year2023::SOLUTIONS)
];

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "Solutions Advent of Code code challenges: https://adventofcode.com")]
#[command(long_about =
"Solutions Advent of Code code challenges: https://adventofcode.com

Advent of Code https://adventofcode.com is a website with Advent Calendars
with small programming puzzles for a variety of skill sets and skill levels that
can be solved in any programming language."
)]
struct Cli {
    /// Year
    year: Option<u16>,

    /// Day
    day: Option<u16>,
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.day {
        Some(1..=24) | None => {}
        Some(n) => {
            eprintln!("Day {} out of range, must be between 1..24.", n);
            return;
        }
    }

    let years: &[(u16, &'static Year)] = match cli.year {
        Some(year) => {
            let year_solutions = YEARS
                .iter()
                .find(|solutions| solutions.0 == year)
                .expect(format!("No solutions found for year {}", year).as_str());

            std::slice::from_ref(year_solutions)
        }
        None => &YEARS[..]
    };

    for year in years {
        println!("Year {}", year.0);

        let days: Vec<(u16, &Solution)> = match cli.day {
            Some(day) => {
                let day_solutions = &year.1[(day - 1) as usize];
                vec![(day, day_solutions)]
            }
            None => year.1
                .iter()
                .enumerate()
                .map(|(day_idx, solution)| ((day_idx + 1) as u16, solution))
                .collect::<Vec<_>>()
        };

        for (day, solution) in days {
            let solve = |of: Option<fn() -> i64>| -> String {
                of.map(|f| f().to_string()).unwrap_or_else(|| "unsolved".to_string())
            };

            let part1 = solve(solution.0);
            let part2 = solve(solution.1);

            println!("\tDay: {:2}, part1: {:>10}, part2: {:>10}", day, part1, part2)
        }
    }
}
