use std::usize;

static DATA: &'static str = include_str!("day4.txt");

struct Board {
    data: Vec<char>,
    width: i64,
    height: i64,
}

impl Board {
    fn new(data: &str) -> Board {
        let mut width: Option<usize> = None;
        let mut chars = Vec::new();
        let mut height = 0;
        for line in data.lines() {
            let line = line.trim();
            match width {
                Some(width) => assert_eq!(width, line.len()),
                None => width = Some(line.len()),
            }
            chars.extend(line.chars());
            height += 1;
        }

        Board {
            data: chars,
            width: width.unwrap() as i64,
            height: height as i64,
        }
    }

    fn check(&self, x: i64, y: i64, c: char) -> bool {
        if !(0..self.width).contains(&x) {
            return false;
        }

        if !(0..self.height).contains(&y) {
            return false;
        }

        let i = (self.width * y + x) as usize;
        self.data.get(i).filter(|cc| **cc == c).is_some()
    }

    fn check_right(&self, x: i64, y: i64) -> bool {
        if !self.check(x + 1, y, 'M') {
            return false;
        }

        if !self.check(x + 2, y, 'A') {
            return false;
        }

        if !self.check(x + 3, y, 'S') {
            return false;
        }

        true
    }

    fn check_down_right(&self, x: i64, y: i64) -> bool {
        if !self.check(x + 1, y + 1, 'M') {
            return false;
        }

        if !self.check(x + 2, y + 2, 'A') {
            return false;
        }

        if !self.check(x + 3, y + 3, 'S') {
            return false;
        }

        true
    }

    fn check_down(&self, x: i64, y: i64) -> bool {
        if !self.check(x, y + 1, 'M') {
            return false;
        }

        if !self.check(x, y + 2, 'A') {
            return false;
        }

        if !self.check(x, y + 3, 'S') {
            return false;
        }

        true
    }

    fn check_down_left(&self, x: i64, y: i64) -> bool {
        if !self.check(x - 1, y + 1, 'M') {
            return false;
        }

        if !self.check(x - 2, y + 2, 'A') {
            return false;
        }

        if !self.check(x - 3, y + 3, 'S') {
            return false;
        }

        true
    }

    fn check_left(&self, x: i64, y: i64) -> bool {
        if !self.check(x - 1, y, 'M') {
            return false;
        }

        if !self.check(x - 2, y, 'A') {
            return false;
        }

        if !self.check(x - 3, y, 'S') {
            return false;
        }

        true
    }

    fn check_left_up(&self, x: i64, y: i64) -> bool {
        if !self.check(x - 1, y - 1, 'M') {
            return false;
        }

        if !self.check(x - 2, y - 2, 'A') {
            return false;
        }

        if !self.check(x - 3, y - 3, 'S') {
            return false;
        }

        true
    }

    fn check_up(&self, x: i64, y: i64) -> bool {
        if !self.check(x, y - 1, 'M') {
            return false;
        }

        if !self.check(x, y - 2, 'A') {
            return false;
        }

        if !self.check(x, y - 3, 'S') {
            return false;
        }

        true
    }

    fn check_up_right(&self, x: i64, y: i64) -> bool {
        if !self.check(x + 1, y - 1, 'M') {
            return false;
        }

        if !self.check(x + 2, y - 2, 'A') {
            return false;
        }

        if !self.check(x + 3, y - 3, 'S') {
            return false;
        }

        true
    }

    fn check_d1(&self, x: i64, y: i64) -> bool {
        (self.check(x - 1, y - 1, 'M') && self.check(x + 1, y + 1, 'S'))
            || (self.check(x - 1, y - 1, 'S') && self.check(x + 1, y + 1, 'M'))
    }

    fn check_d2(&self, x: i64, y: i64) -> bool {
        (self.check(x - 1, y + 1, 'M') && self.check(x + 1, y - 1, 'S'))
            || (self.check(x - 1, y + 1, 'S') && self.check(x + 1, y - 1, 'M'))
    }

    fn check_all_part1(&self) -> i64 {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.check(x, y, 'X') {
                    continue;
                }

                if self.check_right(x, y) {
                    count += 1;
                }

                if self.check_down_right(x, y) {
                    count += 1;
                }

                if self.check_down(x, y) {
                    count += 1;
                }

                if self.check_down_left(x, y) {
                    count += 1;
                }

                if self.check_left(x, y) {
                    count += 1;
                }

                if self.check_left_up(x, y) {
                    count += 1;
                }

                if self.check_up(x, y) {
                    count += 1;
                }

                if self.check_up_right(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    fn check_all_part2(&self) -> i64 {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.check(x, y, 'A') {
                    continue;
                }

                if self.check_d1(x, y) && self.check_d2(x, y) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn part1() -> i64 {
    Board::new(DATA).check_all_part1()
}

pub fn part2() -> i64 {
    Board::new(DATA).check_all_part2()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2662);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2034);
    }

    #[test]
    fn test_example_part1() {
        let data = "MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX\n";
        assert_eq!(Board::new(data).check_all_part1(), 18);
    }

    #[test]
    fn test_example_part2() {
        let data = "MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX\n";
        assert_eq!(Board::new(data).check_all_part2(), 9);
    }
}
