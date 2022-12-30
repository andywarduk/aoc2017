use std::{collections::HashMap, error::Error};

use aoc::input::parse_input_line;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(3, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));

    Ok(())
}

fn part1(target: u64) -> u64 {
    let (x, y) = spiral_to(target);

    x.unsigned_abs() + y.unsigned_abs()
}

fn part2(target: u64) -> u64 {
    spiral_load(target)
}

enum Dir {
    Right,
    Up,
    Left,
    Down,
}

fn spiral_to(target: u64) -> (i64, i64) {
    let mut x = 0;
    let mut y = 0;
    let mut count = 1;
    let mut dir = Dir::Right;
    let mut length = 1;

    'outer: loop {
        for _ in 0..2 {
            for _ in 0..length {
                if count == target {
                    break 'outer;
                }

                match dir {
                    Dir::Right => x += 1,
                    Dir::Up => y += 1,
                    Dir::Left => x -= 1,
                    Dir::Down => y -= 1,
                }

                count += 1;
            }

            dir = match dir {
                Dir::Right => Dir::Up,
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Right,
            };
        }

        length += 1;
    }

    (x, y)
}

fn spiral_load(target: u64) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::Right;
    let mut length = 1;
    let mut values = HashMap::new();

    values.insert((0, 0), 1);

    'outer: loop {
        for _ in 0..2 {
            for _ in 0..length {
                match dir {
                    Dir::Right => x += 1,
                    Dir::Up => y += 1,
                    Dir::Left => x -= 1,
                    Dir::Down => y -= 1,
                }

                let value = [
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                    (x - 1, y),
                    (x + 1, y),
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                ]
                .into_iter()
                .filter_map(|coord| values.get(&coord))
                .cloned()
                .sum::<u64>();

                if value > target {
                    break 'outer value;
                }

                values.insert((x, y), value);
            }

            dir = match dir {
                Dir::Right => Dir::Up,
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Right,
            };
        }

        length += 1;
    }
}

// Input parsing

fn input_transform(line: String) -> u64 {
    line.parse::<u64>().expect("Invalid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(1), 0);
        assert_eq!(part1(12), 3);
        assert_eq!(part1(23), 2);
        assert_eq!(part1(1024), 31);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(1), 2);
        assert_eq!(part2(12), 23);
        assert_eq!(part2(23), 25);
        assert_eq!(part2(750), 806);
    }
}
