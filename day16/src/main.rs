use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

use aoc::input::parse_input_line;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(16, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input, 16));
    println!("Part 2: {}", part2(&input, 16));

    Ok(())
}

fn part1(input: &[Move], prog_cnt: usize) -> String {
    let mut programs = (0..prog_cnt)
        .map(|p| b'a' + p as u8)
        .collect::<VecDeque<_>>();

    for mv in input {
        match mv {
            Move::Spin(n) => programs.rotate_right(*n),
            Move::Exchange(i, j) => programs.swap(*i, *j),
            Move::Partner(a, b) => {
                let i = programs
                    .iter()
                    .position(|p| p == a)
                    .expect("Position a not found");
                let j = programs
                    .iter()
                    .position(|p| p == b)
                    .expect("Position b not found");
                programs.swap(i, j);
            }
        }
    }

    programs.iter().map(|b| *b as char).collect::<String>()
}

const P2ITERS: usize = 1_000_000_000;

fn part2(input: &[Move], prog_cnt: usize) -> String {
    let mut programs = (0..prog_cnt)
        .map(|p| b'a' + p as u8)
        .collect::<VecDeque<_>>();

    let mut orders = HashMap::new();
    let mut dances: usize = 0;
    let mut scanning = true;

    loop {
        for mv in input {
            match mv {
                Move::Spin(n) => programs.rotate_right(*n),
                Move::Exchange(i, j) => programs.swap(*i, *j),
                Move::Partner(a, b) => {
                    let i = programs
                        .iter()
                        .position(|p| p == a)
                        .expect("Position a not found");
                    let j = programs
                        .iter()
                        .position(|p| p == b)
                        .expect("Position b not found");
                    programs.swap(i, j);
                }
            }
        }

        dances += 1;

        if dances == P2ITERS {
            break;
        }

        if scanning {
            if let Some(loop_start) = orders.insert(programs.clone(), dances) {
                let cycle = dances - loop_start;
                dances += ((P2ITERS - dances) / cycle) * cycle;

                scanning = false;
            }
        }
    }

    programs.iter().map(|b| *b as char).collect::<String>()
}

// Input parsing

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

fn input_transform(line: String) -> Vec<Move> {
    line.split(',')
        .map(|ins| match ins.chars().next().expect("No start char") {
            's' => Move::Spin(ins[1..].parse::<usize>().expect("Invalid spin")),
            'x' => {
                let nums = ins[1..]
                    .split('/')
                    .map(|n| n.parse::<usize>().expect("Invalid exchange"))
                    .collect::<Vec<_>>();
                Move::Exchange(nums[0], nums[1])
            }
            'p' => {
                let chars = ins[1..].chars().collect::<Vec<_>>();
                Move::Partner(chars[0] as u8, chars[2] as u8)
            }
            _ => panic!("Invalid instruction"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "s1,x3/4,pe/b";

    #[test]
    fn test1() {
        let input = input_transform(EXAMPLE1.to_string());
        assert_eq!(part1(&input, 5), "baedc");
    }
}
