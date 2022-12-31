use std::{collections::VecDeque, error::Error};

use aoc::input::parse_input_line;

mod knot_hash;

use knot_hash::knot_hash;

const DIMENSION: usize = 128;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(14, input_transform)?;

    let map = calc_map(&input);

    // Run parts
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    Ok(())
}

fn part1(map: &[Vec<bool>]) -> usize {
    map.iter()
        .map(|row| row.iter().filter(|c| **c).count())
        .sum()
}

#[derive(PartialEq, Eq)]
enum Region {
    None,
    Unalloc,
    Region,
}

fn part2(map: &[Vec<bool>]) -> u64 {
    let mut regions: Vec<Vec<Region>> = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| if *c { Region::Unalloc } else { Region::None })
                .collect()
        })
        .collect();

    let mut region_cnt = 0;

    for y in 0..DIMENSION {
        for x in 0..DIMENSION {
            if regions[y][x] == Region::Unalloc {
                region_cnt += 1;
                walk_region(&mut regions, x, y);
            }
        }
    }

    region_cnt
}

fn calc_map(input: &str) -> Vec<Vec<bool>> {
    let mut result = Vec::new();

    for i in 0..DIMENSION {
        let hash = knot_hash(&format!("{}-{}", input, i));

        let bin_string = hash.iter().fold(String::new(), |string, b| {
            string + format!("{:08b}", *b).as_str()
        });

        result.push(
            bin_string
                .chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => panic!("Invalid binary char"),
                })
                .collect(),
        )
    }

    result
}

fn walk_region(map: &mut Vec<Vec<Region>>, x: usize, y: usize) {
    let mut queue = VecDeque::new();

    let add_pos =
        |map: &mut Vec<Vec<Region>>, queue: &mut VecDeque<(usize, usize)>, x: usize, y: usize| {
            if map[y][x] == Region::Unalloc {
                map[y][x] = Region::Region;
                queue.push_back((x, y));
            }
        };

    add_pos(map, &mut queue, x, y);

    while let Some((x, y)) = queue.pop_front() {
        if x > 0 {
            add_pos(map, &mut queue, x - 1, y);
        }
        if x < DIMENSION - 1 {
            add_pos(map, &mut queue, x + 1, y);
        }
        if y > 0 {
            add_pos(map, &mut queue, x, y - 1);
        }
        if y < DIMENSION - 1 {
            add_pos(map, &mut queue, x, y + 1);
        }
    }
}

// Input parsing

fn input_transform(line: String) -> String {
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "flqrgnkx";

    #[test]
    fn test1() {
        let map = calc_map(EXAMPLE1);

        assert_eq!(part1(&map), 8108);
        assert_eq!(part2(&map), 1242);
    }
}
