use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(12, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> usize {
    let map = input
        .iter()
        .map(|i| (i.program, i.conns.clone()))
        .collect::<HashMap<_, _>>();

    let mut visited = HashSet::new();

    walk(&map, 0, &mut visited)
}

fn part2(input: &[InputEnt]) -> usize {
    let map = input
        .iter()
        .map(|i| (i.program, i.conns.clone()))
        .collect::<HashMap<_, _>>();

    let mut visited = HashSet::new();
    let mut groups = 0;

    for p in input {
        if !visited.contains(&p.program) {
            walk(&map, p.program, &mut visited);
            groups += 1;
        }
    }

    groups
}

fn walk(map: &HashMap<u16, Vec<u16>>, node: u16, visited: &mut HashSet<u16>) -> usize {
    let mut count = 1;

    visited.insert(node);

    for c in map[&node].iter() {
        if !visited.contains(c) {
            count += walk(map, *c, visited);
        }
    }

    count
}

// Input parsing

struct InputEnt {
    program: u16,
    conns: Vec<u16>,
}

fn input_transform(line: String) -> InputEnt {
    let mut iter = line.split_whitespace();

    let program = iter
        .next()
        .expect("No program number")
        .parse::<u16>()
        .expect("Invalid program number");

    assert_eq!(iter.next().expect("No join symbol"), "<->");

    let conns = iter
        .map(|c| {
            c.trim_end_matches(',')
                .parse::<u16>()
                .expect("Invalid connection number")
        })
        .collect();

    InputEnt { program, conns }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 6);
        assert_eq!(part2(&input), 2);
    }
}
