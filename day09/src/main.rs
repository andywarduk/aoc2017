use std::error::Error;

use aoc::input::parse_input_line;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(9, input_transform)?;

    let (_, score, garbage) = parse(&input);

    // Run parts
    println!("Part 1: {}", score);
    println!("Part 2: {}", garbage);

    Ok(())
}

enum State {
    Parse,
    Garbage,
    Ignore,
}

fn parse(input: &[char]) -> (u64, u64, u64) {
    let mut state = State::Parse;
    let mut group_count = 0;
    let mut depth = 0;
    let mut score = 0;
    let mut garbage = 0;

    for c in input {
        state = match state {
            State::Parse => match c {
                '{' => {
                    depth += 1;
                    group_count += 1;
                    score += depth;
                    State::Parse
                }
                '}' => {
                    depth -= 1;
                    State::Parse
                }
                ',' => State::Parse,
                '<' => State::Garbage,
                _ => panic!("Invalid character during parse {c}"),
            },
            State::Garbage => match c {
                '>' => State::Parse,
                '!' => State::Ignore,
                _ => {
                    garbage += 1;
                    state
                }
            },
            State::Ignore => State::Garbage,
        };
    }

    (group_count, score, garbage)
}

// Input parsing

fn input_transform(line: String) -> Vec<char> {
    line.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "{}";
    const EXAMPLE2: &str = "{{{}}}";
    const EXAMPLE3: &str = "{{},{}}";
    const EXAMPLE4: &str = "{{{},{},{{}}}}";
    const EXAMPLE5: &str = "{<{},{},{{}}>}";
    const EXAMPLE6: &str = "{<a>,<a>,<a>,<a>}";
    const EXAMPLE7: &str = "{{<a>},{<a>},{<a>},{<a>}}";
    const EXAMPLE8: &str = "{{<!>},{<!>},{<!>},{<a>}}";

    fn test_parse(string: &str, exp_count: u64, exp_score: u64, exp_garbage: u64) {
        let (group_count, score, garbage) = parse(&string.chars().collect::<Vec<_>>());
        assert_eq!(group_count, exp_count);
        assert_eq!(score, exp_score);
        assert_eq!(garbage, exp_garbage);
    }

    #[test]
    fn test1() {
        test_parse(EXAMPLE1, 1, 1, 0);
        test_parse(EXAMPLE2, 3, 6, 0);
        test_parse(EXAMPLE3, 3, 5, 0);
        test_parse(EXAMPLE4, 6, 16, 0);
        test_parse(EXAMPLE5, 1, 1, 10);
        test_parse(EXAMPLE6, 1, 1, 4);
        test_parse(EXAMPLE7, 5, 9, 4);
        test_parse(EXAMPLE8, 2, 3, 13);
    }
}
