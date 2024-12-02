fn load() -> Vec<Vec<i16>> {
    let data = include_str!("day2.txt");

    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i16>().unwrap())
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>()
}

fn is_safe(levels: &[i16]) -> bool {
    levels.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0])))
        || levels.windows(2).all(|w| (1..=3).contains(&(w[0] - w[1])))
}

fn is_safe_remove(levels: &[i16], buffer: &mut Vec<i16>) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        buffer.clear();

        for j in 0..levels.len() {
            if i == j {
                continue;
            }
            buffer.push(levels[j]);
        }

        if is_safe(&buffer[..]) {
            return true;
        }
    }

    false
}

pub fn part1() -> i64 {
    let data = load();
    data.into_iter().filter(|l| is_safe(&l[..])).count() as i64
}

pub fn part2() -> i64 {
    let data = load();
    let mut buffer = Vec::with_capacity(10);
    data.into_iter().filter(|l| is_safe_remove(&l[..], &mut buffer)).count() as i64
}
