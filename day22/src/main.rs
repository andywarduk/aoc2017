use std::error::Error;

use aoc::input::parse_input_vec;

mod board1;
mod board2;
mod dir;
mod pos;

use board1::Board1;
use board2::Board2;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(22, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> usize {
    let mut board = Board1::new(input);

    board.steps(10_000);

    board.infections()
}

fn part2(input: &[InputEnt]) -> usize {
    let mut board = Board2::new(input);

    board.steps(10_000_000);

    board.infections()
}

// Input parsing

type InputEnt = Vec<bool>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("Invalid char"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "..#
#..
...
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let mut board = Board1::new(&input);
        board.steps(7);
        assert_eq!(board.infections(), 5);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let mut board = Board1::new(&input);
        board.steps(70);
        assert_eq!(board.infections(), 41);
    }

    #[test]
    fn test3() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let mut board = Board1::new(&input);
        board.steps(10_000);
        assert_eq!(board.infections(), 5_587);
    }
}
