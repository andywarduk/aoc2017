use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(4, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> usize {
    input
        .iter()
        .filter(|words| {
            words
                .iter()
                .enumerate()
                .all(|(i, word1)| words.iter().skip(i + 1).all(|word2| word1 != word2))
        })
        .count()
}

fn part2(input: &[InputEnt]) -> usize {
    let sorted = input
        .iter()
        .map(|words| {
            words
                .iter()
                .map(|word| {
                    let mut chars = word.chars().collect::<Vec<_>>();
                    chars.sort();
                    chars.into_iter().collect::<String>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&sorted)
}

// Input parsing

type InputEnt = Vec<String>;

fn input_transform(line: String) -> InputEnt {
    line.split_whitespace()
        .map(String::from)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa";

    const EXAMPLE2: &str = "abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        assert_eq!(part2(&input), 3);
    }
}
