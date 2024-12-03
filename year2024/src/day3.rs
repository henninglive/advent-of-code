use regex::Regex;

enum Instruction {
    Mul(i64, i64),
    Enable,
    Disable,
}

fn load() -> Vec<Instruction> {
    let data = include_str!("day3.txt");
    let mut numbers = Vec::new();

    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    for captures in re.captures_iter(data) {
        let capture = captures.get(0).unwrap().as_str();

        if capture == "do()" {
            numbers.push(Instruction::Enable);
        } else if capture == "don't()" {
            numbers.push(Instruction::Disable);
        } else {
            let capture = &capture[4..capture.len() - 1];
            let mut split: std::str::Split<'_, char> = capture.split(',');
            let n1 = split.next().unwrap().parse::<i64>().unwrap();
            let n2 = split.next().unwrap().parse::<i64>().unwrap();
            numbers.push(Instruction::Mul(n1, n2));
        }
    }
    numbers
}

pub fn part1() -> i64 {
    load()
        .into_iter()
        .filter_map(|i| match i {
            Instruction::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum()
}

pub fn part2() -> i64 {
    load()
        .into_iter()
        .fold((0, true), |(count, enabled), i| match i {
            Instruction::Mul(a, b) => (count + if enabled { a * b } else { 0 }, enabled),
            Instruction::Enable => (count, true),
            Instruction::Disable => (count, false),
        })
        .0
}
