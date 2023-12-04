use std::ops::RangeInclusive;

static NUMERIC_ASCII_RANGE: RangeInclusive<u8> = b'0' ..= b'9';

static NUMBER_NAMES: [(&'static [u8], u8); 9] = [
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6),
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9)
];


fn load() -> Vec<&'static str> {
    let data = include_str!("day1.txt");
    data.lines().collect()
}

fn part1_find_first(s: &'static str) -> Option<i64> {
     s
        .chars()
        .find(|c| c.is_numeric())
        .and_then(|i| i.to_digit(10))
        .map(|i| i as i64)
}

fn part1_find_last(s: &'static str) -> Option<i64> {
    s
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .and_then(|i| i.to_digit(10))
        .map(|i| i as i64)
}

pub fn part1() -> i64 {
    load()
        .into_iter()
        .map(|s| {
            let first = part1_find_first(s).unwrap();
            let last = part1_find_last(s).unwrap();
            first * 10 + last
        })
        .sum()
}


fn part2_find_first(s: &'static str) -> Option<i64> {
    let data = s.as_bytes();

    for i in 0..data.len() {
        if NUMERIC_ASCII_RANGE.contains(&data[i]) {
            return Some((data[i] - b'0') as i64);
        }
        
        for number in NUMBER_NAMES {
            let end = i + number.0.len();
            if end > data.len() {
                continue;
            }
            let target = &data[i..end];
            if target == number.0 {
                return Some(number.1 as i64);
            }
        }
    }
    None
}

fn part2_find_last(s: &'static str) -> Option<i64> {
    let data = s.as_bytes();

    for i in (0..data.len()).rev() {
        if NUMERIC_ASCII_RANGE.contains(&data[i]) {
            return Some((data[i] - b'0') as i64);
        }
        
        for number in NUMBER_NAMES {
            let start = (i as isize + 1) - number.0.len() as isize;
            if start < 0 {
                continue;
            }
            let start = start as usize;
            let target = &data[start..(i + 1)];
            if target == number.0 {
                return Some(number.1 as i64);
            }
        }
    }

    None
}


pub fn part2() -> i64 {
    load()
        .into_iter()
        .map(|s: &str| {
            let first = part2_find_first(s).unwrap();
            let last = part2_find_last(s).unwrap();
            first * 10 + last
        })
        .sum()
}

#[test]
fn test_part1_find_first() {
    assert_eq!(part1_find_first("test"), None);
    assert_eq!(part1_find_first("1abc2"), Some(1));
    assert_eq!(part1_find_first("pqr3stu8vwx"), Some(3));
    assert_eq!(part1_find_first("a1b2c3d4e5f"), Some(1));
    assert_eq!(part1_find_first("treb7uchet"), Some(7));
}

#[test]
fn test_part1_find_last() {
    assert_eq!(part1_find_last("test"), None);
    assert_eq!(part1_find_last("1abc2"), Some(2));
    assert_eq!(part1_find_last("pqr3stu8vwx"), Some(8));
    assert_eq!(part1_find_last("a1b2c3d4e5f"), Some(5));
    assert_eq!(part1_find_last("treb7uchet"), Some(7));
}

#[test]
fn test_part2_find_first() {
    assert_eq!(part2_find_first("test"), None);
    assert_eq!(part2_find_first("two1nine"), Some(2));
    assert_eq!(part2_find_first("eightwothree"), Some(8));
    assert_eq!(part2_find_first("abcone2threexyz"), Some(1));
    assert_eq!(part2_find_first("xtwone3four"), Some(2));
    assert_eq!(part2_find_first("4nineeightseven2"), Some(4));
    assert_eq!(part2_find_first("zoneight234"), Some(1));
    assert_eq!(part2_find_first("7pqrstsixteen"), Some(7));
}

#[test]
fn test_part2_find_last() {
    assert_eq!(part2_find_last("test"), None);
    assert_eq!(part2_find_last("two1nine"), Some(9));
    assert_eq!(part2_find_last("eightwothree"), Some(3));
    assert_eq!(part2_find_last("abcone2threexyz"), Some(3));
    assert_eq!(part2_find_last("xtwone3four"), Some(4));
    assert_eq!(part2_find_last("4nineeightseven2"), Some(2));
    assert_eq!(part2_find_last("zoneight234"), Some(4));
    assert_eq!(part2_find_last("7pqrstsixteen"), Some(6));
}



#[test]
fn test_part1() {
    assert_eq!(part1(), 55621);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 53592);
}
