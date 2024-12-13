static DATA: &'static str = include_str!("day10.txt");

struct Map {
    tiles: Vec<u8>,
    width: i16,
    height: i16,
}

impl Map {
    fn load(data: &str) -> Map {
        let mut tiles = Vec::new();
        let mut width: Option<usize> = None;
        let mut height = 0;

        for line in data.lines() {
            match width {
                Some(width) => assert_eq!(width, line.len()),
                None => width = Some(line.len()),
            }

            tiles.extend(line.chars().filter_map(|c| c.to_digit(10)).map(|i| i as u8));

            height += 1;
        }

        Map {
            tiles,
            height,
            width: width.unwrap() as i16,
        }
    }

    fn get_cell(&self, x: i16, y: i16) -> Option<u8> {
        if x < 0 || x >= self.width {
            return None;
        }

        if y < 0 || y >= self.height {
            return None;
        }

        let i = y as usize * self.width as usize + x as usize;
        Some(self.tiles[i])
    }

    fn count_recursive(&self, heads: &mut Vec<(i16, i16)>, x: i16, y: i16, lvl: u8) -> i16 {
        if let Some(cell_lvl) = self.get_cell(x, y) {
            if lvl == cell_lvl {
                if lvl == 9 {
                    let xy = (x, y);
                    if !heads.contains(&xy) {
                        heads.push(xy);
                    }
                    return 1;
                }

                let next_lvl = lvl + 1;

                let mut score = 0;
                score += self.count_recursive(heads, x, y - 1, next_lvl);
                score += self.count_recursive(heads, x + 1, y, next_lvl);
                score += self.count_recursive(heads, x, y + 1, next_lvl);
                score += self.count_recursive(heads, x - 1, y, next_lvl);
                return score;
            }
        }
        return 0;
    }

    fn scores(&self) -> i64 {
        let mut score: i64 = 0;
        let mut heads = Vec::<(i16, i16)>::new();

        for y in 0..self.height {
            for x in 0..self.width {
                heads.clear();
                self.count_recursive(&mut heads, x, y, 0);
                score += heads.len() as i64;
            }
        }

        score
    }

    fn ratings(&self) -> i64 {
        let mut score: i64 = 0;
        let mut heads = Vec::<(i16, i16)>::new();

        for y in 0..self.height {
            for x in 0..self.width {
                heads.clear();
                score += self.count_recursive(&mut heads, x, y, 0) as i64;
            }
        }

        score
    }
}

fn solve_part1(data: &str) -> i64 {
    let map = Map::load(data);
    map.scores()
}

fn solve_part2(data: &str) -> i64 {
    let map = Map::load(data);
    map.ratings()
}

pub fn part1() -> i64 {
    solve_part1(&DATA)
}

pub fn part2() -> i64 {
    solve_part2(&DATA)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = "89010123\n\
        78121874\n\
        87430965\n\
        96549874\n\
        45678903\n\
        32019012\n\
        01329801\n\
        10456732";

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part1(EXAMPLE), 36);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part2(EXAMPLE), 81);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 744);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1651);
    }
}
