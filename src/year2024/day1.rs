fn load() -> (Vec<i64>, Vec<i64>) {
    let data = include_str!("day1.txt");
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in data.lines() {
        let mut parts = line.split_whitespace();
        let n1 = parts.next().unwrap().parse::<i64>().unwrap();
        left.push(n1);

        let n2 = parts.next().unwrap().parse::<i64>().unwrap();
        right.push(n2);
    }

    (right, left)
}

pub fn part1() -> i64 {
    let (mut left, mut right) = load();

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(n1, n2)| (n1 - n2).abs())
        .sum()
}

pub fn part2() -> i64 {
    let (left, right) = load();

    left.into_iter()
        .map(|l| {
            let count = right.iter().filter(|r| **r == l).count();
            l * count as i64
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 1646452);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 23609874);
}
