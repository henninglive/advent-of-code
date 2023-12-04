
#[derive(Debug, PartialEq, Eq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    rounds: Vec<Round>
}


impl Game {
    fn parse(s: &str) -> Game {
        static PREFIX: &'static str = "Game "; 

        let mut main_parts = s.split(':');
        let part_1: &str =  main_parts.next().expect("cound not parse main_part_1");
        let part_2 =  main_parts.next().expect("cound not parse main_part_2");

        let part_1 = part_1.trim_start_matches(PREFIX);
        let id = part_1.parse::<u32>().expect("cound not parse id");
        
        let mut rounds = vec![];

        for round in part_2.split(';') {
            let mut red: u32 = 0u32;
            let mut green: u32 = 0u32;
            let mut blue: u32 = 0u32;

            for color_part in round.split(',') {
                let mut parts = color_part.split_whitespace();
                let count: &str =  parts.next().expect("cound not parse count");
                let count = count.parse::<u32>().expect("cound not parse count to u32");

                let color: &str =  parts.next().expect("cound not parse color");
                match color {
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    s => panic!("unexpected color {}", s)
                }
            }

            rounds.push(Round {
                red,
                green,
                blue
            });

        }

        Game {
            id,
            rounds
        }
    }
}


fn load() -> Vec<Game> {
    include_str!("day2.txt")
        .lines()
        .map(|l| Game::parse(l))
        .collect::<Vec<_>>()
}

pub fn part1() -> i64 {
    load()
        .into_iter()
        .filter(|g| g.rounds.iter().all(|r| r.red <= 12 && r.green <= 13 && r.blue <= 14))
        .map(|g| g.id)
        .sum::<u32>()
        as i64
}

pub fn part2() -> i64 {
    load()
        .into_iter()
        .map(|g| {            
            let red = g.rounds.iter().map(|r| r.red).max().unwrap();
            let green = g.rounds.iter().map(|r| r.green).max().unwrap();
            let blue = g.rounds.iter().map(|r| r.blue).max().unwrap();
            (red * green * blue) as i64
        })
        .sum::<i64>()
}

#[test]
fn test_parse() {
    assert_eq!(
        Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        Game {
            id: 1,
            rounds: vec![
                Round {red: 4, green: 0, blue: 3},
                Round {red: 1, green: 2, blue: 6},
                Round {red: 0, green: 2, blue: 0},
            ]
        }
    );
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 2101);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 58269);
}
