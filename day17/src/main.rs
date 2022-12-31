use std::{collections::VecDeque, error::Error};

use aoc::input::parse_input_line;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(17, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));

    Ok(())
}

const P1ITERS: usize = 2017;

fn part1(input: usize) -> usize {
    let mut buf = VecDeque::with_capacity(P1ITERS + 1);

    buf.push_front(0);

    for i in 1..=P1ITERS {
        buf.rotate_left((input + 1) % buf.len());
        buf.push_front(i);
    }

    buf[1]
}

const P2ITERS: usize = 50_000_000;

fn part2(input: usize) -> usize {
    let mut zero_pos: isize = 0;
    let shift = input as isize;
    let mut last = 0;

    for i in 1..=P2ITERS {
        zero_pos = (zero_pos - 1 - shift).rem_euclid(i as isize) + 1;

        if zero_pos as usize == i {
            last = i;
        }
    }

    last
}

// Input parsing

fn input_transform(line: String) -> usize {
    line.parse::<usize>().expect("Invalid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(3), 638);
        assert_eq!(part2(0), 0 /* TODO */);
    }
}
