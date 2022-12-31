use std::{collections::HashMap, error::Error};

use aoc::input::parse_input_vec;

use crate::input::{input_transform, parse_program};

mod input;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(25, input_transform)?;

    let program = parse_program(&input);

    // Run parts
    println!("Part 1: {}", part1(&program));

    Ok(())
}

fn part1(program: &Program) -> usize {
    let mut cur_state = program.init_state;
    let mut tape_pos = 0;
    let mut tape: HashMap<isize, bool> = HashMap::new();

    for _ in 0..program.diag_steps {
        let state = program
            .states
            .get(&cur_state)
            .expect("State does not exist");

        let tape_val = tape.entry(tape_pos).or_insert(false);

        let action = if *tape_val { &state.one } else { &state.zero };

        *tape_val = action.write;

        tape_pos += match action.mv {
            Dir::Left => -1,
            Dir::Right => 1,
        };

        cur_state = action.next;
    }

    tape.values().filter(|v| **v).count()
}

#[derive(Debug)]
pub struct Program {
    init_state: char,
    diag_steps: u64,
    states: HashMap<char, State>,
}

#[derive(Debug)]
pub struct State {
    zero: Action,
    one: Action,
}

#[derive(Debug)]
pub struct Action {
    write: bool,
    mv: Dir,
    next: char,
}

#[derive(Debug)]
pub enum Dir {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        let program = parse_program(&input);

        assert_eq!(part1(&program), 0 /* TODO */);
    }
}
