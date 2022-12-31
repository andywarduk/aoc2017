use std::{collections::VecDeque, error::Error};

use itertools::Itertools;

use aoc::input::parse_input_line;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(10, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input, 255));
    println!("Part 2: {}", part2(&input, 255));

    Ok(())
}

fn part1(input: &str, elem_max: u8) -> u64 {
    let twists = input
        .split(',')
        .map(|e| e.parse::<usize>().expect("Failed to parse number"))
        .collect::<Vec<_>>();

    let mut queue = (0..=elem_max).collect::<VecDeque<u8>>();
    let mut rotated = 0;
    let mut skip_size = 0;

    twist(&twists, &mut queue, &mut skip_size, &mut rotated);

    queue.rotate_right(rotated % queue.len());

    queue[0] as u64 * queue[1] as u64
}

fn part2(input: &str, elem_max: u8) -> String {
    let twists = input
        .as_bytes()
        .iter()
        .map(|b| *b as usize)
        .chain([17, 31, 73, 47, 23].into_iter())
        .collect::<Vec<_>>();

    let mut queue = (0..=elem_max).collect::<VecDeque<u8>>();
    let mut rotated = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        twist(&twists, &mut queue, &mut skip_size, &mut rotated);
    }

    queue.rotate_right(rotated % queue.len());

    let bytes = queue
        .iter()
        .chunks(16)
        .into_iter()
        .map(|chunk| chunk.fold(0, |acc, byte| acc ^ *byte))
        .collect::<Vec<_>>();

    bytes.iter().map(|b| format!("{:02x}", b)).join("")
}

fn twist(twists: &[usize], queue: &mut VecDeque<u8>, skip_size: &mut usize, rotated: &mut usize) {
    for twist in twists.iter() {
        let mut new_queue = queue.split_off(*twist);
        new_queue.extend(queue.iter().rev());
        *queue = new_queue;
        queue.rotate_left(*skip_size % queue.len());
        *rotated += *twist + *skip_size;
        *skip_size += 1;
    }
}

// Input parsing

fn input_transform(line: String) -> String {
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1("3,4,1,5", 4), 12);
    }
}
