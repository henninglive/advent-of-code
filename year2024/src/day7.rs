
static DATA: &'static str = include_str!("day7.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator 
{
    Add,
    Multiply,
    Concatenation
}

impl Operator {
    fn increment(&mut self, concatenation: bool) -> bool {
        match (*self, concatenation) {
            (Operator::Add, _) => {
                *self = Operator::Multiply;
                false
            },
            (Operator::Multiply, false) => {
                *self = Operator::Add;
                true
            }
            (Operator::Multiply, true) => {
                *self = Operator::Concatenation;
                false
            }
            (Operator::Concatenation, false) => panic!(),
            (Operator::Concatenation, true) => {
                *self = Operator::Add;
                true
            }
        }
    }

    fn solve(&self, a: i64, b: i64) -> i64 {
        match *self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenation => {
                let s = (b as f64).log10().floor() as i32;
                let e = f64::powi(10.0f64, s + 1) as i64;
                a * e + b
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OperatorIteratorStatus{
    First,
    Continue,
    Done
}

struct OperatorIterator(Vec<Operator>, OperatorIteratorStatus, bool);

impl OperatorIterator {
    fn new(size: usize, concatenation: bool) -> OperatorIterator {
        OperatorIterator(vec![Operator::Add; size], OperatorIteratorStatus::First, concatenation)
    }

    fn next(&mut self) -> Option<&[Operator]> {
        match self.1 {
            OperatorIteratorStatus::Done => return None,
            OperatorIteratorStatus::First => {
                self.1 = OperatorIteratorStatus::Continue;
                return Some(&self.0[..]);
            }
            _ => (),
        }

        let mut idx: usize = self.0.len() - 1;

        loop {
            let carry = self.0[idx].increment(self.2);
            if !carry {
                break;
            }
            
            if idx == 0 {
                self.1 = OperatorIteratorStatus::Done;
                return None;
            }

            idx -= 1
        }

        Some(&self.0[..])
    }
}



fn load(data: &str) -> Vec<(i64, Vec<i64>)>{
    data
    .lines()
    .map(|line| {

        let line = line.trim();
        let mut split = line.split(':');
        let first = split.next().unwrap().parse::<i64>().unwrap();
        let other = split.next().unwrap();
        
        let numbers = other
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        (first, numbers)
    })
    .collect::<Vec<(i64, Vec<i64>)>>()
}

fn solve(data: &str, concatenation: bool) -> i64 {
    let mut sum = 0;

    for equation in load(data) {
        let mut iter = OperatorIterator::new(equation.1.len() - 1, concatenation);
        while let Some(operators) = iter.next() {
            let mut numbers = equation.1.iter();
            let first = numbers.next().unwrap();
            let mut ops = operators.iter();

            let result = numbers.fold(*first, |a, b| ops.next().unwrap().solve(a, *b));
            if result == equation.0 {
                sum += result;
                break;
            }
        }
    }

    sum
}

pub fn part1() -> i64 {
    solve(&DATA, false)
}

pub fn part2() -> i64 {
    solve(&DATA, true)
}


#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = "190: 10 19\n\
        3267: 81 40 27\n\
        83: 17 5\n\
        156: 15 6\n\
        7290: 6 8 6 15\n\
        161011: 16 10 13\n\
        192: 17 8 14\n\
        21037: 9 7 18 13\n\
        292: 11 6 16 20";
    
    #[test]
    fn test_load_example() {
        let list = load(EXAMPLE);
        assert_eq!(list.len(), 9);

        let first: &(i64, Vec<i64>) = &list[0];
        assert_eq!(first.0, 190);
        assert_eq!(first.1, &[10, 19]);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve(EXAMPLE, false), 3749);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve(EXAMPLE, true), 11387);
    }


    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1611660863222);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 945341732469724);
    }

}