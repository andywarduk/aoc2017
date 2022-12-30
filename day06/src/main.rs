use std::{collections::HashSet, error::Error};

use aoc::input::parse_input_line;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(6, input_transform)?;

    let (part1, part2) = run(input);

    // Run parts
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn run(mut banks: Vec<u64>) -> (u64, u64) {
    let mut states = HashSet::new();

    let mut count = 0;

    let mut first = true;

    let mut part1 = 0;
    let part2;

    states.insert(banks.clone());

    loop {
        let max = banks.iter().max().expect("Max should be found");

        let elem = banks
            .iter()
            .position(|x| *x == *max)
            .expect("Max elem should be found");

        let mut blocks = banks[elem];
        banks[elem] = 0;

        let mut alloc_elem = elem;

        while blocks > 0 {
            alloc_elem = (alloc_elem + 1) % banks.len();
            banks[alloc_elem] += 1;
            blocks -= 1;
        }

        count += 1;

        if !states.insert(banks.clone()) {
            if first {
                part1 = count;
                states.clear();
                states.insert(banks.clone());
                count = 0;
                first = false;
            } else {
                part2 = count;
                break;
            }
        }
    }

    (part1, part2)
}

// Input parsing

fn input_transform(line: String) -> Vec<u64> {
    line.split_whitespace()
        .map(|s| s.parse::<u64>().expect("Invalid number"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "0 2 7 0";

    #[test]
    fn test1() {
        let input = input_transform(EXAMPLE1.to_string());

        let (part1, part2) = run(input);

        assert_eq!(part1, 5);
        assert_eq!(part2, 4);
    }
}
