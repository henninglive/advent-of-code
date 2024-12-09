static DATA: &'static str = include_str!("day6.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Obstacle,
    Visited,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn offset(&self) -> (i16, i16) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    pos: (i16, i16),
    dir: Direction,
}

impl Map {
    fn load(data: &str) -> Map {
        let mut tiles = Vec::new();
        let mut width = None;
        let mut pos = None;

        for (y, line) in data.lines().enumerate() {
            let line = line.trim();
            match width {
                Some(width) => assert_eq!(width, line.len()),
                None => width = Some(line.len()),
            }

            for (x, c) in line.char_indices() {
                match c {
                    '.' => tiles.push(Tile::Empty),
                    '#' => tiles.push(Tile::Obstacle),
                    '^' => {
                        assert!(pos.is_none());
                        tiles.push(Tile::Empty);
                        pos = Some((x as i16, y as i16));
                    }
                    _ => panic!("unexpected character '{}'", c),
                }
            }
        }

        let width = width.unwrap();

        Map {
            width,
            height: tiles.len() / width,
            tiles,
            pos: pos.unwrap(),
            dir: Direction::Up,
        }
    }

    fn step(&mut self) -> bool {
        if self.pos.0 < 0 || self.pos.0 >= self.width as i16 {
            return false;
        }

        if self.pos.1 < 0 || self.pos.1 >= self.height as i16 {
            return false;
        }

        let offset = self.dir.offset();
        let next = (self.pos.0 + offset.0, self.pos.1 + offset.1);
        let idx = next.1 as usize * self.width + next.0 as usize;

        if let Some(Tile::Obstacle) = self.tiles.get(idx) {
            self.dir = self.dir.turn();
        } else {
            let prev: usize = self.pos.1 as usize * self.width + self.pos.0 as usize;
            self.tiles[prev] = Tile::Visited;
            self.pos = next;
        }

        true
    }

    fn count_steps(&self) -> usize {
        self.tiles.iter().filter(|t| **t == Tile::Visited).count()
    }
}

fn solve_part1(data: &str) -> i64 {
    let mut map = Map::load(data);
    while map.step() {}
    map.count_steps() as i64
}

pub fn part1() -> i64 {
    solve_part1(&DATA)
}

pub fn part2() -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = "....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...";

    #[test]
    fn test_load_example() {
        let map = Map::load(EXAMPLE);
        assert_eq!(map.width, 10);
        assert_eq!(map.tiles.len(), 100);
        assert_eq!(
            map.tiles.iter().filter(|t| **t == Tile::Obstacle).count(),
            8
        );
        assert_eq!(map.pos, (4, 6));
        assert_eq!(map.dir, Direction::Up);
        assert_eq!(map.count_steps(), 0);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 41);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 4819);
    }
}
