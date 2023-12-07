use std::ops::RangeInclusive;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};


static NUMERIC_ASCII_RANGE: RangeInclusive<u8> = b'0' ..= b'9';


fn load() -> Vec<&'static [u8]> {
    include_str!("day3.txt")
        .lines()
        .map(|l| l.as_bytes())
        .collect()
}


#[derive(Debug)]
struct Number {
    value: i64,
    start: usize,
    end: usize
}

#[derive(Debug)]
struct Parser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl <'a> Parser<'a> {
    fn new(data: &'a [u8]) -> Parser<'a> {
        Parser {
            data,
            pos: 0
        }
    }
}


impl <'a> std::iter::Iterator for Parser<'a >{
    type Item = Number;

    fn next(&mut self) -> Option<Number> {
        while self.pos < self.data.len() && !NUMERIC_ASCII_RANGE.contains(&self.data[self.pos]) {
            self.pos += 1;
        }

        if self.pos >= self.data.len() {
            return None;
        }

        let start = self.pos; 
        while self.pos < self.data.len() && NUMERIC_ASCII_RANGE.contains(&self.data[self.pos]) {
            self.pos += 1;
        }
        let end = self.pos;
        debug_assert!(end > start);
        debug_assert!(end <= self.data.len());

        let data = &self.data[start..end];
        let data_str = std::str::from_utf8(data).unwrap();
        let value = data_str.parse::<i64>().unwrap();
        

        Some(Number {
            value,
            start,
            end
        })
    }
}

#[derive(Debug, Eq)]
struct SymbolPointer<'a>(&'a u8);


impl <'a> PartialEq<SymbolPointer<'a>> for SymbolPointer<'a> {
    fn eq(&self, other: &SymbolPointer<'a>) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl <'a> Hash for SymbolPointer<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.0 as *const u8 as usize)
    }
}


fn is_symbol(b: u8) -> bool {
    if b == b'.' {
        return false
    }

   if NUMERIC_ASCII_RANGE.contains(&b) {
        return false;
   }

   true
}

fn find_neighboring_symbols<'a>(num: &Number, center: &'a [u8], top: Option<&'a [u8]>, bottom: Option<&'a [u8]>) -> Vec<&'a u8> {
    if let Some(top) = top {
        assert_eq!(center.len(), top.len());
    }

    if let Some(bottom) = bottom {
        assert_eq!(center.len(), bottom.len());
    }

    let mut symbols = vec![];

    // check if there is a symbol to the right
    if num.start >= 1 && is_symbol(center[num.start - 1]) {
        symbols.push(&center[num.start - 1]);
    }

    // check if there is a symbol to the left
    if num.end + 1 <= center.len() && is_symbol(center[num.end]) {
        symbols.push(&center[num.end]);
    }

    // check if there is a symbol to the over
    if let Some(top) = top {
        let start = if num.start == 0 {0} else {num.start - 1};
        let end = if num.end == top.len() {num.end} else {num.end + 1};
        for b in &top[start..end] {
            if is_symbol(*b) {
                symbols.push(b);
            }
        }
    }

    // check if there is a symbol to the under
    if let Some(bottom) = bottom {
        let start = if num.start == 0 {0} else {num.start - 1};
        let end = if num.end == bottom.len() {num.end} else {num.end + 1};
        for b in &bottom[start..end] {
            if is_symbol(*b) {
                symbols.push(b);
            }
        }
    }

    symbols
}


pub fn part1() -> i64 {
    let lines = load();

    let mut sum = 0;

    for i in 0..lines.len() {
        let center = lines[i];
        let over = if i == 0 {None} else {Some(lines[i - 1])};
        let under = if i == lines.len() - 1 {None} else {Some(lines[i + 1])};
        let numbers = Parser::new(center).collect::<Vec<_>>();
        for n in numbers {
            if !find_neighboring_symbols(&n, center, over, under).is_empty() {
                sum += n.value
            }
        }
    }


    sum
}

pub fn part2() -> i64 {
    let lines = load();

    let mut symbol_groups: HashMap<SymbolPointer<>, Vec<i64>> = HashMap::new();

    for i in 0..lines.len() {
        let center = lines[i];
        let over = if i == 0 {None} else {Some(lines[i - 1])};
        let under = if i == lines.len() - 1 {None} else {Some(lines[i + 1])};
        let numbers = Parser::new(center).collect::<Vec<_>>();

        for n in numbers {
            let neighboring = find_neighboring_symbols(&n, center, over, under);
            for neighbor in neighboring {
                if neighbor != &b'*' {
                    continue;
                }

                let key = SymbolPointer(neighbor);
                let group = symbol_groups.entry(key).or_insert_with(|| vec![]);
                group.push(n.value);
            }
        }
    }

    symbol_groups
        .into_iter()
        .filter(|(_, values)| values.len() >= 2)
        .map(|(_, values)| {
            assert_eq!(values.len(), 2);
            values.into_iter().product::<i64>()
        })
        .sum()
}


#[test]
fn test_parser() {
    let result = Parser::new("467..114..".as_bytes()).collect::<Vec<_>>();
    assert_eq!(result.len(), 2);

    assert_eq!(result[0].start, 0);
    assert_eq!(result[0].end, 3);
    assert_eq!(result[0].value, 467);

    assert_eq!(result[1].start, 5);
    assert_eq!(result[1].end, 8);
    assert_eq!(result[1].value, 114);
}


#[test]
fn test_has_no_symbol() {
    let over  = None;
    let center = "..592.....".as_bytes();
    let under = None;
    let numbers = Parser::new(center).collect::<Vec<_>>();
    assert_eq!(numbers.len(), 1);

    let symbols = find_neighboring_symbols(&numbers[0], center, over, under);
    assert_eq!(symbols.len(), 0);
}

#[test]
fn test_has_symbol_right() {
    let over  = None;
    let center = "617*......".as_bytes();
    let under = None;
    let numbers = Parser::new(center).collect::<Vec<_>>();
    assert_eq!(numbers.len(), 1);

    let symbols = find_neighboring_symbols(&numbers[0], center, over, under);
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0], &b'*');
}

#[test]
fn test_has_symbol_left() {
    let over  = None;
    let center = "....*617......".as_bytes();
    let under = None;
    let numbers = Parser::new(center).collect::<Vec<_>>();

    let symbols = find_neighboring_symbols(&numbers[0], center, over, under);
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0], &b'*');
}

#[test]
fn test_has_symbol_over() {

    let over  = Some(".....*....".as_bytes());
    let center = ".664.598..".as_bytes();
    let under = None;
    let numbers = Parser::new(center).collect::<Vec<_>>();
    assert_eq!(numbers.len(), 2);

    let symbols_0 = find_neighboring_symbols(&numbers[0], center, over, under);
    assert_eq!(symbols_0.len(), 0);

    let symbols_1 = find_neighboring_symbols(&numbers[1], center, over, under);
    assert_eq!(symbols_1.len(), 1);
    assert_eq!(symbols_1[0], &b'*');
}

#[test]
fn test_has_symbol_under() {
    let over  = None;
    let center = "467..114..".as_bytes();
    let under = Some("...*......".as_bytes());
    let numbers = Parser::new(center).collect::<Vec<_>>();
    assert_eq!(numbers.len(), 2);

    let symbols_0 = find_neighboring_symbols(&numbers[0], center, over, under);
    assert_eq!(symbols_0.len(), 1);
    assert_eq!(symbols_0[0], &b'*');

    let symbols_1 = find_neighboring_symbols(&numbers[1], center, over, under);
    assert_eq!(symbols_1.len(), 0);
}



#[test]
fn test_part1() {
    assert_eq!(part1(), 551094);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 80179647);
}

