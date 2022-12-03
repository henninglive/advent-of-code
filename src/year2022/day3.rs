fn load() -> Vec<&'static str> {
    include_str!("day3.txt")
        .lines()
        .inspect(|line| assert!(line.chars().all(|c| c.is_ascii_alphabetic())))
        .collect::<Vec<_>>()
}

fn char_value(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("Unexpected character '{}'", c),
    }
}

fn char_bitset(s: &str) -> u64 {
    let mut bitset = 0;
    for c in s.chars() {
        bitset |= 1 << char_value(c);
    }
    bitset
}

fn bitset_sum(bitset: u64) -> i64 {
    let mut sum = 0;
    for i in 0..53 {
        if bitset & (1 << i) > 0 {
            sum += i;
        }
    }
    sum
}

pub fn part1() -> i64 {
    load()
        .into_iter()
        .map(|line: &str| -> i64 {
            assert_eq!(line.len() % 2, 0);
            assert!(line.len() >= 2);
            let compartments = line.split_at(line.len() / 2);
            bitset_sum(char_bitset(compartments.0) & char_bitset(compartments.1))
        })
        .sum::<i64>()
}

pub fn part2() -> i64 {
    let rucksacks = load();
    assert_eq!(rucksacks.len() % 3, 0);

    rucksacks
        .chunks_exact(3)
        .map(|chunk| {
            let a = char_bitset(chunk[0]);
            let b = char_bitset(chunk[1]);
            let c = char_bitset(chunk[2]);
            let common = a & b & c;
            assert_eq!(1, common.count_ones());
            bitset_sum(common)
        })
        .sum::<i64>()
}

#[test]
fn test_char_value() {
    assert_eq!(1, char_value('a'));
    assert_eq!(26, char_value('z'));
    assert_eq!(27, char_value('A'));
    assert_eq!(52, char_value('Z'));
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 7674);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 2805);
}
