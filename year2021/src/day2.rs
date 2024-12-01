enum Action {
    FORWARD,
    UP,
    DOWN,
}

fn parse(s: &str) -> (Action, i64) {
    let mut split = s.split_whitespace();
    let op = match split.next() {
        Some("forward") => Action::FORWARD,
        Some("up") => Action::UP,
        Some("down") => Action::DOWN,
        Some(s) => panic!("unknown action '{}'", s),
        None => panic!("No action"),
    };

    let arg = split.next().unwrap().parse::<i64>().unwrap();

    (op, arg)
}

fn load() -> Vec<(Action, i64)> {
    include_str!("day2.txt")
        .lines()
        .map(|s| parse(s))
        .collect::<Vec<_>>()
}

pub fn part1() -> i64 {
    let mut x = 0;
    let mut y = 0;
    for (op, arg) in load() {
        match op {
            Action::FORWARD => x += arg,
            Action::UP => y -= arg,
            Action::DOWN => y += arg,
        }
    }
    x * y
}

pub fn part2() -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (op, arg) in load() {
        match op {
            Action::FORWARD => {
                x += arg;
                y += aim * arg;
            }
            Action::UP => aim -= arg,
            Action::DOWN => aim += arg,
        }
    }
    x * y
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 1692075);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 1749524700);
}
