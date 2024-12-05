static DATA: &'static str = include_str!("day5.txt");

struct Manual {
    ordering: Vec<(i64, i64)>,
    updates: Vec<Vec<i64>>,
}

impl Manual {
    fn load(data: &str) -> Manual {
        let mut lines = data.lines();

        let mut ordering = Vec::new();
        for line in lines.by_ref().take_while(|l| !l.is_empty()) {
            let mut parts = line.split('|');
            let n1 = parts.next().unwrap().parse::<i64>().unwrap();
            let n2 = parts.next().unwrap().parse::<i64>().unwrap();
            ordering.push((n1, n2));
        }

        let updates = lines
            .map(|line| {
                line.split(',')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>();

        Manual { ordering, updates }
    }

    fn find_rules(&self, n: i64) -> Vec<i64> {
        self.ordering
            .iter()
            .filter(|o| o.1 == n)
            .map(|o| o.0)
            .collect()
    }

    fn part1(&self) -> i64 {
        let mut sum = 0;
        for update in self.updates.iter() {
            let find_index = |n: i64| {
                update
                    .iter()
                    .enumerate()
                    .find(|(_, m)| n == **m)
                    .map(|(j, _)| j)
            };

            let check_rules = |i: usize, n: i64| {
                self.find_rules(n)
                    .into_iter()
                    .filter_map(|m| find_index(m))
                    .all(|j| j < i)
            };

            let is_valid = update.iter().enumerate().all(|(i, j)| check_rules(i, *j));

            if is_valid {
                assert!(update.len() % 2 == 1);
                sum += update[update.len() / 2];
            }
        }

        sum
    }
}

pub fn part1() -> i64 {
    Manual::load(DATA).part1()
}

pub fn part2() -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let data = "47|53\n\
            97|13\n\
            97|61\n\
            97|47\n\
            75|29\n\
            61|13\n\
            75|53\n\
            29|13\n\
            97|29\n\
            53|29\n\
            61|53\n\
            97|53\n\
            61|29\n\
            47|13\n\
            75|47\n\
            97|75\n\
            47|61\n\
            75|61\n\
            47|29\n\
            75|13\n\
            53|13\n\
            \n\
            75,47,61,53,29\n\
            97,61,53,29,13\n\
            75,29,13\n\
            75,97,47,61,53\n\
            61,13,29\n\
            97,13,75,29,47";

        let manual = Manual::load(data);
        assert_eq!(manual.ordering.len(), 21);
        assert_eq!(manual.updates.len(), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 5509);
    }
}
