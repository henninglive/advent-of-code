use std::collections::HashSet;

use bit_vec::BitVec;

static DATA: &'static str = include_str!("day6.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i16,
    y: i16,
    dir: Direction,
}

impl Position {
    fn step(&self) -> Position {
        match self.dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
                dir: self.dir,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
                dir: self.dir,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
                dir: self.dir,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
                dir: self.dir,
            },
        }
    }

    fn turn(&self) -> Position {
        match self.dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y,
                dir: Direction::Right,
            },
            Direction::Right => Position {
                x: self.x,
                y: self.y,
                dir: Direction::Down,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y,
                dir: Direction::Left,
            },
            Direction::Left => Position {
                x: self.x,
                y: self.y,
                dir: Direction::Up,
            },
        }
    }

    fn index(&self, map: &Map) -> usize {
        self.y as usize * map.width + self.x as usize
    }

    fn out_of_bounds(&self, map: &Map) -> bool {
        if self.x < 0 || self.x >= map.width as i16 {
            return true;
        }

        if self.y < 0 || self.y >= map.height as i16 {
            return true;
        }
        return false;
    }
}

struct Map {
    tiles: BitVec,
    width: usize,
    height: usize,
}

fn load(data: &str) -> (Map, Position) {
    let mut tiles = BitVec::new();
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
                '.' => tiles.push(false),
                '#' => tiles.push(true),
                '^' => {
                    assert!(pos.is_none());
                    tiles.push(false);
                    pos = Some(Position {
                        x: x as i16,
                        y: y as i16,
                        dir: Direction::Up,
                    });
                }
                _ => panic!("unexpected character '{}'", c),
            }
        }
    }

    let width = width.unwrap();
    let map = Map {
        width,
        height: tiles.len() / width,
        tiles,
    };
    (map, pos.unwrap())
}

fn step(map: &Map, pos: Position) -> Option<Position> {
    let next = pos.step();
    if next.out_of_bounds(map) {
        return None;
    }

    let idx = next.index(map);
    let tile = map.tiles[idx];
    if tile {
        return Some(pos.turn());
    }

    return Some(next);
}

fn count_distinct_positions(history: &[Position]) -> i64 {
    history
        .iter()
        .map(|p| (p.x, p.y))
        .collect::<HashSet<(i16, i16)>>()
        .len() as i64
}

fn detect_loop(history: &[Position], pos: Position) -> bool {
    history.contains(&pos)
}

fn solve_part1(data: &str) -> i64 {
    let (map, mut pos) = load(data);
    let mut history: Vec<Position> = Vec::new();
    history.push(pos);

    while let Some(new_pos) = step(&map, pos) {
        assert!(!detect_loop(&history, new_pos));
        history.push(new_pos);
        pos = new_pos;
    }

    count_distinct_positions(&history[..])
}

fn solve_part2(data: &str) -> i64 {
    let (mut map, start_pos) = load(data);

    let mut history: Vec<Position> = Vec::new();

    let mut loop_count = 0;
    for y in 0..map.width {
        for x in 0..map.width {
            let idx: usize = y * map.width + x;
            if map.tiles[idx] {
                continue;
            }

            let mut pos = start_pos;
            history.clear();
            history.push(pos);

            map.tiles.set(idx, true);

            while let Some(new_pos) = step(&map, pos) {
                if detect_loop(&history, new_pos) {
                    loop_count += 1;
                    break;
                }

                history.push(new_pos);
                pos = new_pos;
            }

            map.tiles.set(idx, false);
        }
    }

    loop_count
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
        let (map, pos) = load(EXAMPLE);
        assert_eq!(map.width, 10);
        assert_eq!(map.tiles.len(), 100);
        assert_eq!(map.tiles.count_ones(), 8);
        assert_eq!(pos.x, 4);
        assert_eq!(pos.y, 6);
        assert_eq!(pos.dir, Direction::Up);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 41);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 4819);
    }

    // Test is slow
    //#[test]
    #[allow(dead_code)]
    fn test_part2() {
        assert_eq!(part2(), 1796);
    }
}
