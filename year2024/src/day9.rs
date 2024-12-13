
static DATA: &'static str = include_str!("day9.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Block {
    Used(u16),
    Free
}

fn load(data: &str) -> Vec<Block> {
    let data = data.trim();
    let mut blocks = Vec::new();

    let mut id: u16 = 0;
    let mut used = true;
    
    for c in data.chars() {
        let n = c.to_digit(10).unwrap() as usize;

        if used {
            blocks.extend(std::iter::repeat(Block::Used(id)).take(n));
            id += 1;
        } else {
            blocks.extend(std::iter::repeat(Block::Free).take(n));
        }

        used = !used;
    }

    blocks
}

fn defragment_simple(disk: &mut Vec<Block>) {
    let mut i = 0;
    let mut j = disk.len() - 1;
    while i < j {

        if let Block::Used(..) = disk[i]   {
            i += 1;
            continue;
        }
        
        if disk[j] == Block::Free {
            j -= 1;
            continue;
        }

        disk.swap(i, j);
    }
}

fn defragment_chunks(disk: &mut Vec<Block>) {

    //TODO: This is shit code.

    let mut j = usize::MAX;
    'main: loop {

        let mut chunks = disk.chunk_by_mut(|a, b| match (a, b){
            (Block::Used(a), Block::Used(b)) => a == b,
            (Block::Free, Block::Free) => true,
            _ => false
        }).collect::<Vec<_>>(); 

        j = usize::min(j, chunks.len() - 1);

        let (a, b) = chunks.split_at_mut(j);
        let used_chunk: &mut [Block] = &mut b[0];
        if let Block::Free = used_chunk[0] {
            if j == 0 {
                break 'main;
            }

            j -= 1;
            continue 'main;
        }
    
        for i in 0..j {

            let free_chunk = &mut a[i];
            if let Block::Used(..) = free_chunk[0] {
                continue;
            }

            if used_chunk.len() < free_chunk.len() {
                let free_chunk: &mut [Block] = &mut free_chunk[..used_chunk.len()];
                free_chunk.swap_with_slice(used_chunk);
                continue 'main;
            }

            if used_chunk.len() == free_chunk.len() {
                free_chunk.swap_with_slice(used_chunk);

                if j == 0 {
                    break 'main;
                }
                j -= 1;

                continue 'main;
            }
        }

        if j == 0 {
            break 'main;
        }

        j -= 1;
    }
}

fn checksum(disk: &[Block]) -> i64 {
    let mut sum = 0;
    for (i , block) in disk.iter().enumerate() {
        if let Block::Used(id) = block {
            sum += i as i64 * *id as i64;
        }

    }
    sum
}


fn solve_part1(data: &str) -> i64 {
    let mut disk = load(data);
    defragment_simple(&mut disk);
    checksum(&disk)
}

fn solve_part2(data: &str) -> i64 {
    let mut disk = load(data);
    defragment_chunks(&mut disk);
    checksum(&disk)
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

    static EXAMPLE: &'static str = "2333133121414131402";

    #[test]
    fn test_load_1() {
        let blocks = load("1231");
        assert_eq!(
            blocks,
            &[
                Block::Used(0),
                Block::Free,
                Block::Free,
                Block::Used(1),
                Block::Used(1),
                Block::Used(1),
                Block::Free,
            ]
        );
    }

    #[test]
    fn test_load_2() {
        let blocks = load("10101");
        assert_eq!(
            blocks,
            &[
                Block::Used(0),
                Block::Used(1),
                Block::Used(2),
            ]
        );
    }

    #[test]
    fn test_defragment_simple_1() {
        let mut blocks = load("121");
        defragment_simple(&mut blocks);
        assert_eq!(
            blocks,
            &[
                Block::Used(0),
                Block::Used(1),
                Block::Free,
                Block::Free,
            ]
        );
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part1(EXAMPLE), 1928);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part2(EXAMPLE), 2858);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 6382875730645);
    }

    // Test is slow
    //#[test]
    #[allow(dead_code)]
    fn test_part2() {
        assert_eq!(part2(), 6420913943576);
    }
}