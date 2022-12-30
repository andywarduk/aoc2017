use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(5, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));

    Ok(())
}

fn part1(mut input: Vec<i64>) -> u64 {
    let mut ip = 0;
    let mut count = 0;

    loop {
        let new_ip = ip + input[ip as usize];
        input[ip as usize] += 1;
        ip = new_ip;
        count += 1;

        if ip < 0 || ip >= input.len() as i64 {
            break;
        }
    }

    count
}

fn part2(mut input: Vec<i64>) -> u64 {
    let mut ip = 0;
    let mut count = 0;

    loop {
        let new_ip = ip + input[ip as usize];

        input[ip as usize] = if input[ip as usize] >= 3 {
            input[ip as usize] - 1
        } else {
            input[ip as usize] + 1
        };

        ip = new_ip;
        count += 1;

        if ip < 0 || ip >= input.len() as i64 {
            break;
        }
    }

    count
}

// Input parsing

fn input_transform(line: String) -> i64 {
    line.parse::<i64>().expect("Invalid number")
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "0
3
0
1
-3
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(input.clone()), 5);
        assert_eq!(part2(input), 10);
    }
}
