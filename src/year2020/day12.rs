#[derive(Debug, Clone, Copy)]
enum Op {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

struct Ship {
    x: i16,
    y: i16,
    d: Direction,
}

impl Op {
    fn parse(s: &str) -> Result<Op, String> {
        match s {
            "N" => Ok(Op::North),
            "E" => Ok(Op::East),
            "S" => Ok(Op::South),
            "W" => Ok(Op::West),
            "L" => Ok(Op::Left),
            "R" => Ok(Op::Right),
            "F" => Ok(Op::Forward),
            _ => Err(format!("unexpected op {}", s))
        }
    }
}

impl Direction {
    fn translate(self, degrees: i16) -> Direction {
        assert_eq!(degrees % 90, 0);
        let dir = (degrees / 90 + self as i16).rem_euclid(4);
        match dir {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("unexpected value {}", dir),
        }
    }
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            d: Direction::East,
        }
    }

    fn next(self, op: Op, num: i16) -> Ship {
        match (op, self.d) {
            (Op::North, _) | (Op::Forward, Direction::North) => Ship { y: self.y - num, ..self },
            (Op::East, _) | (Op::Forward, Direction::East) => Ship { x: self.x + num, ..self },
            (Op::South, _) | (Op::Forward, Direction::South) => Ship { y: self.y + num, ..self },
            (Op::West, _) | (Op::Forward, Direction::West) => Ship { x: self.x - num, ..self },
            (Op::Left, _) => Ship { d: self.d.translate(-num), ..self },
            (Op::Right, _) => Ship { d: self.d.translate(num), ..self },
        }
    }
}

fn load() -> Vec<(Op, i16)> {
    include_str!("day12.txt").lines()
        .map(|s| {
            let (op_str, num_str) = s.split_at(1);
            (Op::parse(op_str).unwrap(), num_str.parse::<i16>().unwrap())
        })
        .collect::<Vec<_>>()
}

pub fn part1() -> i64 {
    let mut ship = Ship::new();

    let moves = load();
    for &(op, num) in moves.iter() {
        ship = ship.next(op, num);
    }
    ship.x.abs() as i64 + ship.y.abs() as i64
}


#[test]
fn test_part1() {
    assert_eq!(part1(), 508)
}
