fn load() -> Vec<i16> {
    include_str!("day1.txt")
        .lines()
        .map(|s| s.parse::<i16>().unwrap())
        .collect::<Vec<_>>()
}

pub fn part1() -> i64 {
    load().windows(2).filter(|w| w[1] > w[0]).count() as i64
}

pub fn part2() -> i64 {
    let sums: Vec<i16> = load().windows(3).map(|w| w.iter().sum()).collect();

    sums.windows(2).filter(|w| w[1] > w[0]).count() as i64
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 1475);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 1516);
}
