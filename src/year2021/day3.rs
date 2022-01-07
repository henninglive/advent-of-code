fn parse(s: &str) -> u32 {
    let mut u = 32;
    for (i, c) in s.chars().rev().enumerate() {
        match c {
            '0' => {}
            '1' => u |= 1 << i,
            c => panic!("unexpected character '{}'", c),
        }
    }
    u
}

fn load() -> Vec<u32> {
    include_str!("day3.txt").lines()
        .map(|s| parse(s))
        .collect::<Vec<_>>()
}

fn count_number_of_set_bits(numbers: &[u32], bit_idx: usize) -> usize {
    let mut count = 0;
    for n in numbers {
        count += ((n & (1 << bit_idx)) >> bit_idx) as usize;
    }

    count
}


pub fn part1() -> i64 {
    let numbers = load();

    let gamma = (0usize..12)
        .map(|i| (i, count_number_of_set_bits(&numbers, i) >= numbers.len() / 2))
        .fold(0u32, |acc, (i, bit)| acc | ((bit as u32) << i))
        as i64;

    let epsilon = (0usize..12)
        .map(|i| (i, count_number_of_set_bits(&numbers, i) <= numbers.len() / 2))
        .fold(0u32, |acc, (i, bit)| acc | ((bit as u32) << i))
        as i64;

    gamma * epsilon
}

pub fn part2() -> i64 {
    0
}

#[test]
fn test_parse() {
    assert_eq!(parse("100100111101"), 2365);
}


#[test]
fn test_part1() {
    assert_eq!(part1(), 3009600);
}



