use std::error::Error;

use aoc::input::parse_input_line;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(1, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[u8]) -> u64 {
    input
        .iter()
        .fold((input[input.len() - 1], 0), |(last, sum), next| {
            (
                *next,
                if *next == last {
                    sum + *next as u64
                } else {
                    sum
                },
            )
        })
        .1
}

fn part2(input: &[u8]) -> u64 {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            let other = (i + (input.len() / 2)) % input.len();

            if input[other] == *n {
                Some(*n as u64)
            } else {
                None
            }
        })
        .sum()
}

// Input parsing

fn input_transform(line: String) -> Vec<u8> {
    line.chars().map(|c| c as u8 - b'0').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "1122";
    const EXAMPLE2: &str = "1111";
    const EXAMPLE3: &str = "1234";
    const EXAMPLE4: &str = "91212129";

    const EXAMPLE5: &str = "1212";
    const EXAMPLE6: &str = "1221";
    const EXAMPLE7: &str = "123425";
    const EXAMPLE8: &str = "123123";
    const EXAMPLE9: &str = "12131415";

    #[test]
    fn test1() {
        let input = input_transform(EXAMPLE1.to_string());
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test2() {
        let input = input_transform(EXAMPLE2.to_string());
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test3() {
        let input = input_transform(EXAMPLE3.to_string());
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test4() {
        let input = input_transform(EXAMPLE4.to_string());
        assert_eq!(part1(&input), 9);
    }

    #[test]
    fn test5() {
        let input = input_transform(EXAMPLE5.to_string());
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test6() {
        let input = input_transform(EXAMPLE6.to_string());
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test7() {
        let input = input_transform(EXAMPLE7.to_string());
        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn test8() {
        let input = input_transform(EXAMPLE8.to_string());
        assert_eq!(part2(&input), 12);
    }

    #[test]
    fn test9() {
        let input = input_transform(EXAMPLE9.to_string());
        assert_eq!(part2(&input), 4);
    }
}
