use std::collections::HashMap;

static DATA: &'static str = include_str!("day11.txt");

fn base10_digits(n: u64) -> usize {
    (n as f64).log10().floor() as usize + 1
}

fn step(
    stones: HashMap<u64, u64>,
    cache: &mut HashMap<u64, Option<(u64, u64)>>,
) -> HashMap<u64, u64> {
    let mut new = HashMap::<u64, u64>::with_capacity(stones.len());
    for (stone, count) in stones.into_iter() {
        if stone == 0 {
            *new.entry(1).or_default() += count;
            continue;
        }

        let cached = cache.entry(stone).or_insert_with(|| {
            let digets: usize = base10_digits(stone);
            if digets % 2 == 0 {
                let m = 10f64.powi(digets as i32 / 2) as u64;
                Some((stone / m, stone % m))
            } else {
                None
            }
        });

        if let Some((right, left)) = cached {
            *new.entry(*right).or_default() += count;
            *new.entry(*left).or_default() += count;
            continue;
        }

        *new.entry(stone * 2024).or_default() += count;
    }

    new
}

fn load(data: &str) -> HashMap<u64, u64> {
    let stones: Vec<u64> = data
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    stones
        .into_iter()
        .map(|n| (n, 1))
        .collect::<HashMap<u64, u64>>()
}

fn solve(data: &str, steps: usize) -> i64 {
    let mut stones = load(data);

    let mut cache = HashMap::new();
    for _ in 0..steps {
        stones = step(stones, &mut cache);
    }

    stones.into_iter().map(|(_, count)| count as i64).sum()
}

pub fn part1() -> i64 {
    solve(&DATA, 25)
}

pub fn part2() -> i64 {
    solve(&DATA, 75)
}

#[cfg(test)]
mod test {
    use std::iter;

    use super::*;

    static EXAMPLE_1: &'static str = "0 1 10 99 999";
    static EXAMPLE_2: &'static str = "125 17";

    fn map_to_vec(stones: &HashMap<u64, u64>) -> Vec<u64> {
        let mut res = stones
            .into_iter()
            .flat_map(|(stone, count)| iter::repeat(*stone).take(*count as usize))
            .collect::<Vec<u64>>();
        res.sort();
        res
    }

    #[test]
    fn test_example_1() {
        let mut stones = load(EXAMPLE_1);
        let mut cache = HashMap::new();
        stones = step(stones, &mut cache);
        assert_eq!(map_to_vec(&stones), &[0, 1, 1, 9, 9, 2024, 2021976]);
    }

    #[test]
    fn test_example_2_step6() {
        assert_eq!(solve(EXAMPLE_2, 6), 22);
    }

    #[test]
    fn test_example_2_step25() {
        assert_eq!(solve(EXAMPLE_2, 25), 55312);
    }

    #[test]
    fn test_base10_digits() {
        assert_eq!(base10_digits(0), 1);
        assert_eq!(base10_digits(1), 1);
        assert_eq!(base10_digits(10), 2);
        assert_eq!(base10_digits(99), 2);
        assert_eq!(base10_digits(u64::MAX), format!("{}", u64::MAX).len());
    }

    #[test]
    fn test_step() {
        let mut stones = load("10");
        let mut cache = HashMap::new();

        stones = step(stones, &mut cache);
        assert_eq!(map_to_vec(&stones), &[0, 1]);
        stones = step(stones, &mut cache);
        assert_eq!(map_to_vec(&stones), &[1, 2024]);
        stones = step(stones, &mut cache);
        assert_eq!(map_to_vec(&stones), &[20, 24, 2024]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 200446);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 238317474993392);
    }
}
