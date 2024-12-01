fn score_part1(opponent_hand: char, my_hand: char) -> i64 {
    let score_hand = match my_hand {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Unexpected hand '{}'", my_hand),
    };
    let score_winner = match (my_hand, opponent_hand) {
        ('X', 'A') => 3,
        ('Y', 'B') => 3,
        ('Z', 'C') => 3,
        ('X', 'B') => 0,
        ('Y', 'C') => 0,
        ('Z', 'A') => 0,
        ('X', 'C') => 6,
        ('Y', 'A') => 6,
        ('Z', 'B') => 6,
        _ => panic!("Unexpected hands '{}' '{}'", my_hand, opponent_hand),
    };
    score_hand + score_winner
}

fn score_part2(opponent_hand: char, outcome: char) -> i64 {
    let my_hand = match (opponent_hand, outcome) {
        ('A', 'Y') => 'X',
        ('B', 'Y') => 'Y',
        ('C', 'Y') => 'Z',
        ('A', 'X') => 'Z',
        ('B', 'X') => 'X',
        ('C', 'X') => 'Y',
        ('A', 'Z') => 'Y',
        ('B', 'Z') => 'Z',
        ('C', 'Z') => 'X',
        _ => panic!(
            "Unexpected hand or outcome '{}' '{}'",
            opponent_hand, outcome
        ),
    };

    score_part1(opponent_hand, my_hand)
}

fn load() -> Vec<(char, char)> {
    let mut res = Vec::new();

    for line in include_str!("day2.txt").lines() {
        let mut hand_iter = line.split_whitespace().flat_map(|c| c.chars());

        let a = hand_iter.next().unwrap();
        let b = hand_iter.next().unwrap();
        assert!(hand_iter.next().is_none());
        res.push((a, b));
    }

    res
}

pub fn part1() -> i64 {
    load()
        .into_iter()
        .map(|(opponent_hand, my_hand)| score_part1(opponent_hand, my_hand))
        .sum::<i64>()
}

pub fn part2() -> i64 {
    load()
        .into_iter()
        .map(|(opponent_hand, outcome)| score_part2(opponent_hand, outcome))
        .sum::<i64>()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 10310);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 14859);
}
