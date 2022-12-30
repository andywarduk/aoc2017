use std::error::Error;

use itertools::Itertools;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(2, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    input
        .iter()
        .map(|line| {
            let min = line.iter().min().expect("Unable to get minimum");
            let max = line.iter().max().expect("Unable to get maximum");

            max - min
        })
        .sum()
}

fn part2(input: &[InputEnt]) -> u64 {
    input
        .iter()
        .map(|line| {
            line.iter()
                .permutations(2)
                .find_map(|nums| {
                    if nums[0] % nums[1] == 0 {
                        Some(nums[0] / nums[1])
                    } else {
                        None
                    }
                })
                .expect("Clean divisors not found")
        })
        .sum()
}

// Input parsing

type InputEnt = Vec<u64>;

fn input_transform(line: String) -> InputEnt {
    line.split_whitespace()
        .map(|s| s.parse::<u64>().expect("Invalid number"))
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "5 1 9 5
7 5 3
2 4 6 8";

    const EXAMPLE2: &str = "5 9 2 8
9 4 7 3
3 8 6 5";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        assert_eq!(part2(&input), 9);
    }
}
