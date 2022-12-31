use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(24, input_transform)?;

    // Walk the connectors
    let mut result = WalkResult::default();

    walk(State::new(input.to_vec()), &mut result);

    // Report results
    println!("Part 1: {}", result.strongest);
    println!("Part 2: {}", result.longest_strength);

    Ok(())
}

#[derive(Clone)]
struct Conn {
    port_a: u8,
    port_b: u8,
}

#[derive(Default)]
struct WalkResult {
    strongest: u64,
    longest: usize,
    longest_strength: u64,
}

#[derive(Clone)]
struct State {
    available: Vec<Conn>,
    last: u8,
    length: usize,
    strength: u64,
}

impl State {
    fn new(available: Vec<Conn>) -> Self {
        Self {
            available,
            last: 0,
            length: 0,
            strength: 0,
        }
    }
}

fn walk(state: State, result: &mut WalkResult) {
    // Find connectors
    let suitable = state
        .available
        .iter()
        .enumerate()
        .filter(|(_, conn)| conn.port_a == state.last || conn.port_b == state.last)
        .collect::<Vec<_>>();

    if suitable.is_empty() {
        // No connectors available
        if state.strength > result.strongest {
            result.strongest = state.strength;
        }

        if state.length > result.longest {
            result.longest = state.length;
            result.longest_strength = state.strength;
        } else if state.length == result.longest && state.strength > result.longest_strength {
            result.longest_strength = state.strength;
        }
    } else {
        // Iterate
        for (elem, next) in suitable {
            let mut next_state = state.clone();

            next_state.available.swap_remove(elem);

            next_state.last = if next_state.last == next.port_a {
                next.port_b
            } else {
                next.port_a
            };

            next_state.length += 1;
            next_state.strength += next.port_a as u64 + next.port_b as u64;

            walk(next_state, result);
        }
    }
}

// Input parsing

fn input_transform(line: String) -> Conn {
    let nums = line
        .split('/')
        .map(|p| p.parse::<u8>().expect("Invalid number"))
        .collect::<Vec<_>>();

    Conn {
        port_a: nums[0],
        port_b: nums[1],
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        let mut result = WalkResult::default();

        walk(State::new(input.to_vec()), &mut result);

        assert_eq!(result.strongest, 31);
        assert_eq!(result.longest_strength, 19);
    }
}
