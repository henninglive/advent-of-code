fn load() -> Vec<Vec<i64>> {
    let mut all = Vec::new();
    let mut current = Vec::new();

    for line in include_str!("day1.txt").lines() {
        if line.is_empty() {
            all.push(std::mem::replace(&mut current, Vec::new()));
        } else {
            current.push(line.parse::<i64>().unwrap())
        }
    }

    all
}


pub fn part1() -> i64 {
    load()
        .into_iter()
        .map(|elf| elf.into_iter().sum::<i64>())
        .max()
        .unwrap()
}

pub fn part2() -> i64 {
    let mut all = load()
        .into_iter()
        .map(|elf| elf.into_iter().sum::<i64>())
        .collect::<Vec<_>>();

    all.sort_by(|a, b| a.cmp(b).reverse());

    all[0..3].iter().sum::<i64>()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 69693);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 200945);
}
