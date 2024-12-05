use std::collections::BTreeMap;
use std::{fmt, str::FromStr};

static DATA: &'static str = include_str!("day7.txt");

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<Card> for char {
    fn from(c: Card) -> char {
        match c {
            Card::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::Ten => 'T',
            Card::Jack => 'J',
            Card::Queen => 'Q',
            Card::King => 'K',
            Card::Ace => 'A',
        }
    }
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(c: char) -> Result<Self, String> {
        match c {
            'A' | 'a' => Ok(Card::Ace),
            'K' | 'k' => Ok(Card::King),
            'Q' | 'q' => Ok(Card::Queen),
            'J' | 'j' => Ok(Card::Jack),
            'T' | 't' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            c => Err(format!("Invalid character: \"{}\"", c)),
        }
    }
}

impl From<Card> for usize {
    fn from(c: Card) -> usize {
        match c {
            Card::Two => 0,
            Card::Three => 1,
            Card::Four => 2,
            Card::Five => 3,
            Card::Six => 4,
            Card::Seven => 5,
            Card::Eight => 6,
            Card::Nine => 7,
            Card::Ten => 8,
            Card::Jack => 9,
            Card::Queen => 10,
            Card::King => 11,
            Card::Ace => 12,
        }
    }
}

impl TryFrom<usize> for Card {
    type Error = String;

    fn try_from(n: usize) -> Result<Self, String> {
        match n {
            0 => Ok(Card::Two),
            1 => Ok(Card::Three),
            2 => Ok(Card::Four),
            3 => Ok(Card::Five),
            4 => Ok(Card::Six),
            5 => Ok(Card::Seven),
            6 => Ok(Card::Eight),
            7 => Ok(Card::Nine),
            8 => Ok(Card::Ten),
            9 => Ok(Card::Jack),
            10 => Ok(Card::Queen),
            11 => Ok(Card::King),
            12 => Ok(Card::Ace),
            c => Err(format!("Invalid number: \"{}\"", c)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandRanking {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Hand([Card; 5]);

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 5);

        let mut chars = s.chars();
        let mut next = || {
            chars
                .next()
                .ok_or_else(|| "Expected 5 characters".to_string())
                .and_then::<Card, _>(|c| c.try_into())
        };

        let c1 = next()?;
        let c2 = next()?;
        let c3 = next()?;
        let c4 = next()?;
        let c5 = next()?;

        Ok(Hand::new([c1, c2, c3, c4, c5]))
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in self.0 {
            let c: char = card.into();
            write!(f, "{}", c)?
        }
        Ok(())
    }
}

impl Hand {
    fn new(cards: [Card; 5]) -> Hand {
        Hand(cards)
    }

    fn ranking(&self) -> HandRanking {
        let freq: [u8; 13] = self.0.iter().fold([0u8; 13], |mut freq, card| {
            let idx: usize = (*card).into();
            freq[idx] += 1;
            freq
        });

        if freq.iter().any(|f| *f == 5) {
            return HandRanking::FiveOfAKind;
        }

        if freq.iter().any(|f| *f == 4) {
            return HandRanking::FourOfAKind;
        }

        if freq.iter().any(|f| *f == 3) && freq.iter().any(|f| *f == 2) {
            return HandRanking::FullHouse;
        }

        if freq.iter().any(|f| *f == 3) {
            return HandRanking::ThreeOfAKind;
        }

        if freq.iter().filter(|f| **f == 2).count() == 2 {
            return HandRanking::TwoPair;
        }

        if freq.iter().any(|f| *f == 2) {
            return HandRanking::OnePair;
        }

        HandRanking::HighCard
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ranking()
            .cmp(&other.ranking())
            .then_with(|| self.0.cmp(&other.0))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn load(data: &str) -> Vec<(Hand, i64)> {
    let mut hands = Vec::new();
    for line in data.lines() {
        let line = line.trim();
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap().parse::<Hand>().unwrap();
        let bid = parts.next().unwrap().parse::<i64>().unwrap();
        hands.push((hand, bid));
    }

    hands
}

fn calc_part1(data: &str) -> i64 {
    let hands = load(data);
    let hands = hands.into_iter().collect::<BTreeMap<Hand, i64>>();

    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as i64 + 1) * bid)
        .sum()
}

pub fn part1() -> i64 {
    calc_part1(DATA)
}

pub fn part2() -> i64 {
    0
}

#[cfg(test)]
mod test {
    use more_asserts::*;

    use super::*;

    fn parse_cards(str: &str) -> Hand {
        str.parse::<Hand>().unwrap()
    }

    #[test]
    fn test_examples() {
        let data = "32T3K 765\n\
            T55J5 684\n\
            KK677 28\n\
            KTJJT 220\n\
            QQQJA 483";

        assert_eq!(calc_part1(data), 6440);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 251216224);
    }

    #[test]
    fn test_card_ordering() {
        assert!(Card::Five == Card::Five);
        assert!(Card::Ace > Card::King);
        assert!(Card::Three < Card::Four);
    }

    #[test]
    fn test_hand_parse() {
        assert_eq!(
            parse_cards("32T3K"),
            Hand::new([Card::Three, Card::Two, Card::Ten, Card::Three, Card::King])
        );
        assert_eq!(
            parse_cards("KK677"),
            Hand::new([Card::King, Card::King, Card::Six, Card::Seven, Card::Seven])
        );
        assert_eq!(
            parse_cards("KTJJT"),
            Hand::new([Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten])
        );
        assert_eq!(
            parse_cards("T55J5"),
            Hand::new([Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five])
        );
        assert_eq!(
            parse_cards("QQQJA"),
            Hand::new([Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace])
        );
    }

    #[test]
    fn test_hand_display() {
        assert_eq!(
            Hand::new([Card::King, Card::Ten, Card::Three, Card::Three, Card::Two]).to_string(),
            "KT332",
        );
        assert_eq!(
            Hand::new([Card::King, Card::King, Card::Seven, Card::Seven, Card::Six]).to_string(),
            "KK776",
        );
        assert_eq!(
            Hand::new([Card::King, Card::Jack, Card::Jack, Card::Ten, Card::Ten]).to_string(),
            "KJJTT",
        );
        assert_eq!(
            Hand::new([Card::Jack, Card::Ten, Card::Five, Card::Five, Card::Five]).to_string(),
            "JT555",
        );
        assert_eq!(
            Hand::new([Card::Ace, Card::Queen, Card::Queen, Card::Queen, Card::Jack]).to_string(),
            "AQQQJ",
        );
    }

    #[test]
    fn test_hand_ranking() {
        // Five of a Kind

        assert_eq!(parse_cards("AAAAA").ranking(), HandRanking::FiveOfAKind);

        // Four of a Kind

        assert_eq!(parse_cards("KKKKQ").ranking(), HandRanking::FourOfAKind);

        assert_eq!(parse_cards("AQQQQ").ranking(), HandRanking::FourOfAKind);

        // Full house

        assert_eq!(parse_cards("KKK66").ranking(), HandRanking::FullHouse);

        assert_eq!(parse_cards("KK666").ranking(), HandRanking::FullHouse);

        assert_eq!(parse_cards("23332").ranking(), HandRanking::FullHouse,);

        // Three of a Kind

        assert_eq!(parse_cards("KKKJT").ranking(), HandRanking::ThreeOfAKind);

        assert_eq!(parse_cards("KJJJT").ranking(), HandRanking::ThreeOfAKind);

        assert_eq!(parse_cards("KJTTT").ranking(), HandRanking::ThreeOfAKind);

        // Two pair

        assert_eq!(parse_cards("TT996").ranking(), HandRanking::TwoPair,);

        assert_eq!(parse_cards("TT966").ranking(), HandRanking::TwoPair);

        assert_eq!(parse_cards("T9966").ranking(), HandRanking::TwoPair);

        // One Pair

        assert_eq!(parse_cards("88765").ranking(), HandRanking::OnePair);

        assert_eq!(parse_cards("87765").ranking(), HandRanking::OnePair,);

        assert_eq!(parse_cards("87665").ranking(), HandRanking::OnePair,);

        assert_eq!(parse_cards("87655").ranking(), HandRanking::OnePair);

        assert_eq!(parse_cards("87655").ranking(), HandRanking::OnePair);

        // High Card
        assert_eq!(parse_cards("5432A").ranking(), HandRanking::HighCard);
    }

    #[test]
    fn test_hand_ordering() {
        assert_gt!(parse_cards("33332"), parse_cards("2AAAA"));
    }
}
