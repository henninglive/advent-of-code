fn load() -> (Vec<i64>, Vec<i64>) {
    let data = include_str!("day6.txt");
    let mut lines = data.lines();
    let l1 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let l2 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (l1, l2)
}

fn count_winning_races(time: i64, distance: i64) -> i64 {
    (0..time)
        .map(|wait| wait * (time - wait))
        .filter(|d| *d > distance)
        .count() as i64
}

pub fn part1() -> i64 {
    let (time, distance) = load();

    time.into_iter()
        .zip(distance.into_iter())
        .map(|(time, distance)| count_winning_races(time, distance))
        .product()
}

pub fn part2() -> i64 {
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 32076);
}

#[test]
fn test_count_winning_races() {
    assert_eq!(count_winning_races(7, 9), 4);
    assert_eq!(count_winning_races(15, 40), 8);
    assert_eq!(count_winning_races(30, 200), 9);
}
