use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(19, input_transform)?;

    let (part1, part2) = walk(&input, 'Z');

    // Run parts
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn walk(input: &[InputEnt], last: char) -> (String, usize) {
    let mut y: isize = 0;
    let mut x: isize = input[0]
        .iter()
        .position(|c| *c == '|')
        .expect("Start not found") as isize;
    let mut dir = (0, 1);
    let mut result = String::new();
    let mut steps = 1;

    loop {
        x += dir.0;
        y += dir.1;

        steps += 1;

        let c = input[y as usize][x as usize];

        match c {
            'A'..='Z' => {
                result.push(c);

                if c == last {
                    break;
                }
            }
            '+' => {
                // Change direction
                dir = if dir.0 == 0 {
                    [(1, 0), (-1, 0)]
                } else {
                    [(0, 1), (0, -1)]
                }
                .into_iter()
                .find(|new_dir| {
                    matches!(
                        input[(y + new_dir.1) as usize][(x + new_dir.0) as usize],
                        '|' | '-' | '+' | 'A'..='Z'
                    )
                })
                .unwrap();
            }
            '|' | '-' => (),
            _ => panic!("Unexpected char {c}"),
        }
    }

    (result, steps)
}

// Input parsing

type InputEnt = Vec<char>;

fn input_transform(line: String) -> InputEnt {
    line.chars().collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "     |          
     |  +--+    
     A  |  C    
 F---|--|-E---+ 
     |  |  |  D 
     +B-+  +--+ 
                
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let (part1, part2) = walk(&input, 'F');
        assert_eq!(part1, "ABCDEF");
        assert_eq!(part2, 38);
    }
}
