use std::env;

pub type Solution = (Option<fn() -> i64>, Option<fn() -> i64>);
pub type Year = [Solution; 24];

fn print_solution(day: usize, solution: &Solution) {
    let solve = |of: Option<fn() -> i64>| -> String {
        of.map(|f| f().to_string())
            .unwrap_or_else(|| "unsolved".to_string())
    };

    let part1 = solve(solution.0);
    let part2 = solve(solution.1);

    println!(
        "\tDay: {:2}, part1: {:>10}, part2: {:>10}",
        day, part1, part2
    );
}

pub fn menu(year: &Year) -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if !(1..=2).contains(&args.len()) {
        return Err(format!("Usage: advent-of-code <DAY>"));
    }

    if args.len() == 2 {
        let day = args[1]
            .parse::<usize>()
            .map_err(|e| format!("Invalid date: {}", e))?;

        let day = match day {
            1..=24 => Ok(day),
            _ => Err(format!("Day {} out of range, must be between 1..24.", day)),
        }?;

        let solution = year[day - 1];
        print_solution(day, &solution)
    } else {
        for (day, solution) in year.iter().enumerate() {
            print_solution(day + 1, solution)
        }
    };

    Ok(())
}
