use std::collections::HashSet;

static CARD_DATA: &'static str = include_str!("day4.txt");

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: i16,
    winning_numbers: HashSet<u8>,
    my_numbers: HashSet<u8>,
}

fn parse_line(line: &str) -> Card {
    let mut main_parts = line.split(":");
    let prefix_part = main_parts.next().unwrap();
    let numbers_part = main_parts.next().unwrap();

    let prefix = prefix_part
        .trim_start_matches("Card")
        .trim_start_matches(' ');
    let id = prefix.parse::<i16>().unwrap();

    let mut numbers_parts = numbers_part.split("|");
    let winning_numbers_part = numbers_parts.next().unwrap();
    let my_numbers_part = numbers_parts.next().unwrap();

    let winning_numbers = winning_numbers_part
        .split_whitespace()
        .into_iter()
        .filter(|n| n.len() > 0)
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<HashSet<_>>();

    let my_numbers = my_numbers_part
        .split_whitespace()
        .into_iter()
        .filter(|n| n.len() > 0)
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<HashSet<_>>();

    Card {
        id,
        winning_numbers,
        my_numbers,
    }
}

fn solve_part1(card_data: &'static str) -> i64 {
    card_data
        .lines()
        .map(|l| parse_line(l))
        .map(|card| {
            let count = card.my_numbers.intersection(&card.winning_numbers).count() as i64;
            if count == 0 {
                count
            } else {
                1 << count - 1
            }
        })
        .sum::<i64>()
}

pub fn part1() -> i64 {
    solve_part1(CARD_DATA)
}

fn solve_part2(card_data: &'static str) -> i64 {
    let cards = card_data.lines().map(|l| parse_line(l)).collect::<Vec<_>>();

    let mut counts = vec![1i64; cards.len()];

    for i in 0..cards.len() {
        let card = &cards[i];
        let card_count = counts[i];

        let winning_number_count = card.my_numbers.intersection(&card.winning_numbers).count();

        let start = std::cmp::min(cards.len(), i + 1);
        let end = std::cmp::min(cards.len(), i + winning_number_count + 1);
        for j in start..end {
            counts[j] += card_count;
        }
    }

    counts.into_iter().sum()
}

pub fn part2() -> i64 {
    solve_part2(CARD_DATA)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_CARDS: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parse() {
        let line: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17 9 48 53";
        let card = Card {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17].into_iter().collect::<HashSet<_>>(),
            my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
                .into_iter()
                .collect::<HashSet<_>>(),
        };

        assert_eq!(parse_line(line), card);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(TEST_CARDS), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 23028);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(TEST_CARDS), 30);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 9236992);
    }
}
