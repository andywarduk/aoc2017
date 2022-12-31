use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(15, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[u64]) -> u64 {
    let mut g1 = Generator::new(input[0], 16807, 0);
    let mut g2 = Generator::new(input[1], 48271, 0);

    count_matches(&mut g1, &mut g2, 40_000_000)
}

fn part2(input: &[u64]) -> u64 {
    let mut g1 = Generator::new(input[0], 16807, 0x03);
    let mut g2 = Generator::new(input[1], 48271, 0x07);

    count_matches(&mut g1, &mut g2, 5_000_000)
}

fn count_matches(g1: &mut Generator, g2: &mut Generator, iterations: usize) -> u64 {
    let mut matches = 0;

    for _ in 0..iterations {
        let v1 = g1.next();
        let v2 = g2.next();

        if v1 & 0xffff == v2 & 0xffff {
            matches += 1;
        }
    }

    matches
}

struct Generator {
    last: u64,
    factor: u64,
    mask: u64,
}

impl Generator {
    fn new(init: u64, factor: u64, mask: u64) -> Self {
        Generator {
            last: init,
            factor,
            mask,
        }
    }

    fn next(&mut self) -> u64 {
        let mut num = self.last;

        loop {
            num = (num * self.factor) % 2_147_483_647;

            if num & self.mask == 0 {
                break;
            }
        }

        self.last = num;

        num
    }
}

// Input parsing

fn input_transform(line: String) -> u64 {
    line.split_whitespace()
        .nth(4)
        .expect("Number not found")
        .parse::<u64>()
        .expect("Invalid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: [u64; 2] = [65, 8921];

    #[test]
    fn test1() {
        assert_eq!(part1(&EXAMPLE1), 588);
        assert_eq!(part2(&EXAMPLE1), 309);
    }
}
