use std::collections::{BTreeMap, HashSet};


static DATA: &'static str = include_str!("day8.txt");

struct PairIterator<'a, T: 'a>(&'a [T],usize,usize);

impl <'a, T: 'a>  PairIterator<'a, T> {
    fn new(slice: &'a [T]) -> PairIterator<'a, T> {
        PairIterator(slice, 0, 1)
    }
}


impl <'a, T: 'a> Iterator for PairIterator<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.2 >= self.0.len(){
            return None;
        }

        let next = Some((&self.0[self.1], &self.0[self.2]));

        self.2 += 1;
        if self.2 >= self.0.len() {
            self.1 += 1;
            self.2 = self.1 + 1;
        }

        next
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct V2(i16,i16);

impl V2 {
    fn antinodes(a: V2, b: V2, width:i16,height:i16) -> impl Iterator<Item = V2> {
        let dx = b.0 - a.0;
        let dy = b.1 - a.1;

        let mut antinode = b;
        std::iter::from_fn(move || {

            let res = if antinode.0 < 0 || antinode.0 >= width {
                None
            } else if antinode.1 < 0 || antinode.1 >= height {
                None
            } else {
                Some(antinode)
            };
    
            antinode.0 += dx;
            antinode.1 += dy;

            res
        })
    }
  
}

struct Map {
    antennas: BTreeMap<char,Vec<V2>>,
    width: i16,
    height: i16,
}

fn load(data: &str) -> Map {
    let mut antennas = BTreeMap::<char,Vec<V2>>::new();
    let mut height = 0;
    let mut width: Option<i16> = None;

    for (y, line) in data.lines().enumerate() {
        let line = line.trim();

        let mut w = 0;
        for (x, c) in line.char_indices() {
            match c {
                '.' => {},
                c => antennas.entry(c).or_default().push(V2(x as i16, y as i16))
            }
            w += 1;
        }

        match width {
            Some(width) => assert_eq!(width, w),
            None => width = Some(w)
        }

        height += 1;
    }

    Map {
        antennas,
        height,
        width: width.unwrap()
    }
}



impl Map {
    fn find_first_antinode(&self) -> HashSet<V2> {
        let mut antinodes = HashSet::new();
            
        for (_, antennas) in self.antennas.iter() {
            for (v1, v2) in PairIterator::new(&antennas[..]) {
                for a1 in V2::antinodes(*v1, *v2, self.width, self.height).skip(1).take(1) {
                    antinodes.insert(a1);
                }
                
                for a2 in V2::antinodes(*v2, *v1, self.width, self.height).skip(1).take(1) {
                    antinodes.insert(a2);
                }
            }
        }
        
        antinodes
    }

    fn find_all_antinode(&self) -> HashSet<V2> {
        let mut antinodes = HashSet::new();
            
        for (_, antennas) in self.antennas.iter() {
            for (v1, v2) in PairIterator::new(&antennas[..]) {
                
                for a1 in V2::antinodes(*v1, *v2, self.width, self.height) {
                    antinodes.insert(a1);
                }
                
                for a2 in V2::antinodes(*v2, *v1, self.width, self.height) {
                    antinodes.insert(a2);
                }
            }
        }

        antinodes
    }
    
}


fn solve_part1(data: &str) -> i64 {
    let map: Map = load(data);   
    let antinodes = map.find_first_antinode();
    antinodes.len() as i64
}

fn solve_part2(data: &str) -> i64 {
    let map: Map = load(data);   
    let antinodes = map.find_all_antinode();
    antinodes.len() as i64
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

    static EXAMPLE_1: &'static str = "............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............";

    static EXAMPLE_2: &'static str = "T.........\n\
        ...T......\n\
        .T........\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ..........";

    #[test]
    fn test_load_example_1() {
        let map: Map = load(EXAMPLE_1);
        assert_eq!(map.height, 12);
        assert_eq!(map.width, 12);
        assert_eq!(map.antennas.len(), 2);
        
        let zeros = map.antennas.get(&'0').unwrap();
        assert_eq!(zeros, &[V2(8, 1), V2(5, 2), V2(7, 3), V2(4, 4)]);
    }

    #[test]
    fn test_count_first_antinodes() {
        let antennas= vec![V2(8, 8), V2(9, 9)];
        let antennas= BTreeMap::<char,Vec<V2>>::from_iter(std::iter::once(('A', antennas)));
        let map = Map {
            antennas,
            width: 12,
            height: 12
        };

        let mut antinodes = map.find_first_antinode().into_iter().collect::<Vec<_>>();
        antinodes.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
        assert_eq!(antinodes, &[V2(7, 7), V2(10, 10)]);
    }

    #[test]
    fn test_pair_iterator() {
        let slice = &[1, 2, 3];
        let pairs = PairIterator::new(slice).collect::<Vec<_>>();
        assert_eq!(pairs, &[(&1, &2), (&1, &3), (&2, &3)]);
    }

    #[test]
    fn test_pair_iterator_empty() {
        assert_eq!(PairIterator::<i32>::new(&[]).count(), 0);
    }

    #[test]
    fn test_v2_antinodes() {
        let v1= V2(8, 8);
        let v2 = V2(9, 9);
        assert_eq!(V2::antinodes(v1, v2, 12, 12).collect::<Vec<_>>(), &[V2(9, 9), V2(10, 10), V2(11, 11)]);
        assert_eq!(V2::antinodes(v2, v1, 12, 12).collect::<Vec<_>>(), &[V2(8, 8), V2(7, 7), V2(6, 6), V2(5, 5), V2(4, 4), V2(3, 3), V2(2, 2), V2(1, 1), V2(0, 0)]);
    }

    #[test]
    fn test_example_1_part1() {
        assert_eq!(solve_part1(EXAMPLE_1), 14);        
    }

    #[test]
    fn test_example_1_part2() {
        assert_eq!(solve_part2(EXAMPLE_1), 34);        
    }

    #[test]
    fn test_example_2_part2() {
        assert_eq!(solve_part2(EXAMPLE_2), 9);        
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 285);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 944);
    }

}