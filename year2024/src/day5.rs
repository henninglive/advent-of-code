static DATA: &'static str = include_str!("day5.txt");

fn rules_iter<'a>(rules: &'a [(i16, i16)], n: i16) -> impl Iterator<Item = i16> + 'a {
    rules.iter().filter_map(move |o| match o {
        (n1, n2) if *n2 == n => Some(n1.clone()),
        _ => None,
    })
}

fn rules_iter_reverse<'a>(rules: &'a [(i16, i16)], n: i16) -> impl Iterator<Item = i16> + 'a {
    rules.iter().filter_map(move |o| match o {
        (n1, n2) if *n1 == n => Some(n2.clone()),
        _ => None,
    })
}

fn is_valid_update(update: &[i16], rules: &[(i16, i16)]) -> bool {
    for (i, value) in update.iter().enumerate() {
        for j in rules_iter(rules, *value) {
            if update[(i + 1)..].contains(&j) {
                return false;
            }
        }
    }
    true
}

fn find_next_candidate(candidates: &[i16], rules: &[(i16, i16)]) -> Option<usize> {
    'candidates: for (i, n) in candidates.iter().enumerate() {
        for r in rules_iter_reverse(&rules, *n) {
            if candidates.iter().any(|m| r == *m) {
                continue 'candidates;
            }
        }
        return Some(i);
    }
    None
}

fn reorder_update(update: &mut Vec<i16>, rules: &[(i16, i16)]) {
    let mut new = Vec::with_capacity(update.len());

    while update.len() > 0 {
        let next = find_next_candidate(&update[..], rules).expect("No candidate found");

        new.push(update.remove(next));
    }

    new.reverse();

    *update = new;
}

fn center(update: &[i16]) -> i64 {
    assert!(update.len() % 2 == 1);
    update[update.len() / 2] as i64
}

fn load(data: &str) -> (Vec<(i16, i16)>, Vec<Vec<i16>>) {
    let mut lines = data.lines();

    let mut ordering = Vec::new();
    for line in lines.by_ref().take_while(|l| !l.is_empty()) {
        let mut parts = line.split('|');
        let n1 = parts.next().unwrap().parse::<i16>().unwrap();
        let n2 = parts.next().unwrap().parse::<i16>().unwrap();
        ordering.push((n1, n2));
    }

    let updates = lines
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i16>().unwrap())
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>();

    (ordering, updates)
}

fn solve_part1(data: &str) -> i64 {
    let (ordering, updates) = load(data);
    updates
        .into_iter()
        .filter(|u| is_valid_update(u, &ordering))
        .map(|u| center(&u[..]))
        .sum()
}

pub fn part1() -> i64 {
    solve_part1(DATA)
}

fn solve_part2(data: &str) -> i64 {
    let (ordering, updates) = load(data);

    let mut sum = 0;
    for mut update in updates.into_iter() {
        if is_valid_update(&update[..], &ordering) {
            continue;
        }

        reorder_update(&mut update, &ordering);

        //debug_assert!(is_valid_update(&update[..], &ordering), "{:?}", update);

        sum += center(&update[..]);
    }

    sum
}

pub fn part2() -> i64 {
    solve_part2(DATA)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = "47|53\n\
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

    fn assert_reorder(update: &[i16], rules: &[(i16, i16)], expected: &[i16]) {
        let mut v = update.to_vec();
        reorder_update(&mut v, rules);
        assert_eq!(v, expected);
    }

    #[test]
    fn test_load_example() {
        let (ordering, updates) = load(EXAMPLE);
        assert_eq!(ordering.len(), 21);
        assert_eq!(updates.len(), 6);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 143);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 123);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 5509);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 4407);
    }

    #[test]
    fn test_reorder_update() {
        assert_reorder(&[3, 2, 1], &[(1, 2), (2, 3)], &[1, 2, 3]);
        assert_reorder(&[1, 2, 3], &[(3, 2), (2, 1)], &[3, 2, 1]);
    }

    #[test]
    fn test_reorder_examples() {
        let (ordering, _) = load(EXAMPLE);
        assert_reorder(&[75, 97, 47, 61, 53], &ordering[..], &[97, 75, 47, 61, 53]);
        assert_reorder(&[61, 13, 29], &ordering[..], &[61, 29, 13]);
        assert_reorder(&[97, 13, 75, 29, 47], &ordering[..], &[97, 75, 47, 29, 13]);
    }
}
